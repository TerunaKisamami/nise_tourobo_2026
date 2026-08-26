import can
from rclpy.action import ActionServer, GoalResponse
import rclpy
from rclpy.node import Node
from tourobo_2026_mechanisms.constants import *
from std_msgs.msg import String
import os
import sys
import asyncio
import time

from tourobo_2026_interfaces.action import BallShoot
from ah_python_lib.ah_python_can import *
from dyna_interfaces.msg import DynaTarget

CAN_BUS = can.interface.Bus(
    bustype="socketcan", channel="can0", asynchronous=True, bitrate=1000000
)


class BallShootNode(Node):
    def __init__(self):
        super().__init__("ball_shoot_node")
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()

        self.dyna_extpos_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_extpos", 10
        )
        self.dyna_vel_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_vel", 10
        )
        self.dyna_pos_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_pos", 10
        )

        self._action_server = ActionServer(
            self,
            BallShoot,
            "ball_shoot",
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        # 射出モーターの立ち上げ
        #モーターの初期化
        set_enc_vel_mode(SHOOT_ROLLER_1_CAN_ID, CAN_BUS)
        set_enc_vel_mode(SHOOT_ROLLER_2_CAN_ID, CAN_BUS)
        set_enc_vel_mode(SHOOT_ROLLER_3_CAN_ID, CAN_BUS)

        #回転方向設定
        set_motor_rot_dir(SHOOT_ROLLER_1_CAN_ID, 1, CAN_BUS)
        set_motor_rot_dir(SHOOT_ROLLER_2_CAN_ID, 1, CAN_BUS)
        set_motor_rot_dir(SHOOT_ROLLER_3_CAN_ID, 1, CAN_BUS)

        #ゲインの設定
        set_vel_pid_gain(SHOOT_ROLLER_1_CAN_ID, 10.0, 300.0, 0.0, CAN_BUS)
        set_vel_pid_gain(SHOOT_ROLLER_2_CAN_ID, 10.0, 300.0, 0.0, CAN_BUS)
        set_vel_pid_gain(SHOOT_ROLLER_3_CAN_ID, 10.0, 300.0, 0.0, CAN_BUS)


    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn("現在別の処理を実行中です。新しい指令を拒否します。")
            return GoalResponse.REJECT
        self.get_logger().info("新しい指令を受け付けました。")
        return GoalResponse.ACCEPT

    def publish_dyna_extpos(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_extpos_publisher.publish(msg)

    def publish_dyna_vel(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_vel_publisher.publish(msg)

    def publish_dyna_pos(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_pos_publisher.publish(msg)

    async def shoot_ball(self, push_state):

        # ボールを発射する処理を書く
        # 同時に3つのモーターを回す
        set_goal_vel(SHOOT_ROLLER_1_CAN_ID, SHOOT_MOTOR_SPEED, CAN_BUS)
        set_goal_vel(SHOOT_ROLLER_2_CAN_ID, -SHOOT_MOTOR_SPEED, CAN_BUS)
        set_goal_vel(SHOOT_ROLLER_3_CAN_ID, -SHOOT_MOTOR_SPEED, CAN_BUS)

        # ロボマスを使って押し出す
        if push_state != Shoot_Push_State.MAX.value:
            set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MAX, CAN_BUS)
        time.sleep(WAIT_TIME_PUSH_HALF)

        # 射出モーターを停止
        set_goal_vel(SHOOT_ROLLER_1_CAN_ID, 0, CAN_BUS)
        set_goal_vel(SHOOT_ROLLER_2_CAN_ID, 0, CAN_BUS)
        set_goal_vel(SHOOT_ROLLER_3_CAN_ID, 0, CAN_BUS)

        # 押出機構を下限までさげる
        set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MIN, CAN_BUS)
        time.sleep(WAIT_TIME_PUSH)

        return True

    async def execute_callback(self, goal_handle):

        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallShoot.Result()
            success = False

            success = await self.shoot_ball(req.push_state)

            if success:
                res.success = True
                res.next_state = 1  # NOT_CARRY
                goal_handle.succeed()
            else:
                goal_handle.abort()

            return res
        finally:
            self.is_executing = False


def main(args=None):
    rclpy.init(args=args)
    node = BallShootNode()
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()
