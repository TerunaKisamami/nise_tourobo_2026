#!/usr/bin/env python3
import math
import rclpy
from rclpy.node import Node
from rclpy.action import ActionServer, CancelResponse, GoalResponse
from rclpy.callback_groups import ReentrantCallbackGroup
from rclpy.executors import MultiThreadedExecutor

from nav_msgs.msg import Path
from geometry_msgs.msg import Twist
from std_msgs.msg import Float32
from nav2_msgs.action import FollowPath

from tf2_ros import Buffer, TransformListener


class OmniPurePursuitActionServer(Node):

    def __init__(self):
        super().__init__("omni_pure_pursuit_action_server")

        # --- 1. 走行パラメータ ---
        self.declare_parameter("global_frame", "map")
        self.declare_parameter("robot_frame", "base_link")
        self.declare_parameter("max_linear_vel", 2.0)
        self.declare_parameter("accel_limit", 2.0)
        self.declare_parameter("min_lookahead_dist", 0.2)
        self.declare_parameter("lookahead_gain", 0.2)
        self.declare_parameter("max_angular_vel", 1.5)
        self.declare_parameter("kp_yaw", 1.8)  # 旋回をキビキビさせるため少し上げ

        # --- 2. 判定パラメータ ---
        self.declare_parameter("goal_tolerance", 0.01)
        self.declare_parameter("yaw_tolerance", 0.01)
        self.declare_parameter("v_floor", 0.4)
        self.declare_parameter("v_min_push", 0.18)
        self.declare_parameter("min_vel_enforce_dist", 0.15)
        self.declare_parameter("decel_start_dist", 0.5)

        self.declare_parameter("action_name", "follow_path_map")
        action_name = self.get_parameter("action_name").value

        self.tf_buffer = Buffer()
        self.tf_listener = TransformListener(self.tf_buffer, self)
        self.cmd_pub = self.create_publisher(Twist, "/cmd_vel", 10)
        self.v_debug_pub = self.create_publisher(Float32, "/debug/v_final", 10)

        self._action_server = ActionServer(
            self,
            FollowPath,
            action_name,
            execute_callback=self.execute_callback,
            goal_callback=self.goal_callback,
            cancel_callback=self.cancel_callback,
            callback_group=ReentrantCallbackGroup())

        self.get_logger().info(
            f"NHK2026 Pure Pursuit: [Translation/Rotation Split Mode]")

    def goal_callback(self, goal_request):
        return GoalResponse.ACCEPT

    def cancel_callback(self, goal_handle):
        return CancelResponse.ACCEPT

    def get_quaternion_to_euler(self, q):
        return 0.0, 0.0, math.atan2(2.0 * (q.w * q.z + q.x * q.y),
                                    1.0 - 2.0 * (q.y * q.y + q.z * q.z))

    def publish_zero_vel(self):
        self.cmd_pub.publish(Twist())

    def plan_velocity_profile(self, path):
        max_v = self.get_parameter("max_linear_vel").value
        accel = self.get_parameter("accel_limit").value
        num_points = len(path.poses)
        distances = [0.0] * num_points
        total_dist = 0.0
        for i in range(1, num_points):
            p1 = path.poses[i - 1].pose.position
            p2 = path.poses[i].pose.position
            total_dist += math.hypot(p2.x - p1.x, p2.y - p1.y)
            distances[i] = total_dist
        velocity_profile = []
        for s in distances:
            v_accel = math.sqrt(2 * accel * s)
            v_decel = math.sqrt(2 * accel * max(0.0, total_dist - s))
            velocity_profile.append(min(v_accel, max_v, v_decel))
        return velocity_profile

    def get_robot_pose(self):
        try:
            t = self.tf_buffer.lookup_transform(
                self.get_parameter("global_frame").value,
                self.get_parameter("robot_frame").value, rclpy.time.Time())
            _, _, yaw = self.get_quaternion_to_euler(t.transform.rotation)
            return t.transform.translation.x, t.transform.translation.y, yaw
        except:
            return None

    async def execute_callback(self, goal_handle):
        path = goal_handle.request.path
        v_profile = self.plan_velocity_profile(path)
        current_idx = 0
        rate = self.create_rate(20)

        # パラメータ取得
        v_floor = self.get_parameter("v_floor").value
        v_push = self.get_parameter("v_min_push").value
        d_decel = self.get_parameter("decel_start_dist").value
        d_enforce = self.get_parameter("min_vel_enforce_dist").value
        g_tol = self.get_parameter("goal_tolerance").value
        y_tol = self.get_parameter("yaw_tolerance").value

        while rclpy.ok():
            if goal_handle.is_cancel_requested:
                goal_handle.canceled()
                self.publish_zero_vel()
                return FollowPath.Result()

            pose = self.get_robot_pose()
            if not pose:
                rate.sleep()
                continue
            rx, ry, ryaw = pose

            # 1. 最近傍点検索
            min_d = float('inf')
            search_range = min(current_idx + 50, len(path.poses))
            for i in range(current_idx, search_range):
                d = math.hypot(path.poses[i].pose.position.x - rx,
                               path.poses[i].pose.position.y - ry)
                if d < min_d:
                    min_d = d
                    current_idx = i

            # 2. 誤差の計算
            dist_to_goal = math.hypot(path.poses[-1].pose.position.x - rx,
                                      path.poses[-1].pose.position.y - ry)
            _, _, gyaw = self.get_quaternion_to_euler(
                path.poses[-1].pose.orientation)
            yaw_err = math.atan2(math.sin(gyaw - ryaw), math.cos(gyaw - ryaw))

            # 3. 終了判定（両方合格でループを抜ける）
            if dist_to_goal < g_tol and abs(yaw_err) < y_tol:
                break

            # 4. 速度決定ロジック
            v_planned = v_profile[current_idx]

            # --- 【核心】ここを条件分岐を厳格化 ---
            if dist_to_goal < g_tol:
                # 距離が合格なら、並進速度を【物理的に 0】にする
                v_final = 0.0
            elif dist_to_goal < d_enforce:
                # 15cm以内：押し込み速度を適用
                v_final = max(v_planned, v_push)
            else:
                # 巡航中：v_floorを保証して爆走
                v_final = max(v_planned, v_floor)

            # 5. 指令値生成（Twist）
            cmd = Twist()

            # 並進の計算（v_final が 0 ならここは 0 になる）
            if v_final > 0:
                ld = self.get_parameter("min_lookahead_dist").value + (
                    self.get_parameter("lookahead_gain").value * v_final)
                target_pt = path.poses[-1].pose.position
                for i in range(current_idx, len(path.poses)):
                    if math.hypot(path.poses[i].pose.position.x - rx,
                                  path.poses[i].pose.position.y - ry) >= ld:
                        target_pt = path.poses[i].pose.position
                        break

                dx, dy = target_pt.x - rx, target_pt.y - ry
                angle_to_target = math.atan2(dy, dx) - ryaw
                cmd.linear.x = v_final * math.cos(angle_to_target)
                cmd.linear.y = v_final * math.sin(angle_to_target)
            else:
                cmd.linear.x = 0.0
                cmd.linear.y = 0.0

            # 旋回の計算（距離に関わらず、角度がズレていれば常に回る）
            kp_y = self.get_parameter("kp_yaw").value
            max_ang_vel = self.get_parameter("max_angular_vel").value
            z_out = yaw_err * kp_y
            min_z = 0.12  # 旋回不感帯対策
            if abs(yaw_err) > y_tol:
                if abs(z_out) < min_z:
                    z_out = math.copysign(min_z, z_out)
                cmd.angular.z = max(-max_ang_vel, min(max_ang_vel, z_out))
            else:
                cmd.angular.z = 0.0

            # 6. パブリッシュ
            self.cmd_pub.publish(cmd)
            self.v_debug_pub.publish(Float32(data=v_final))

            #self.get_logger().info(
            #    f"D: {dist_to_goal:.3f} | Y: {yaw_err:.3f} | V: {v_final:.2f}",
            #    throttle_duration_sec=0.2)
            rate.sleep()

        self.publish_zero_vel()
        self.get_logger().info("目標到達：座標・角度ともに合格。")
        goal_handle.succeed()
        return FollowPath.Result()


def main(args=None):
    rclpy.init(args=args)
    node = OmniPurePursuitActionServer()
    rclpy.spin(node, executor=MultiThreadedExecutor())
    node.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()
