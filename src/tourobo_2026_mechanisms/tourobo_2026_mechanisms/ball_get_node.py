"""
どういう動きをするか↓

"""
from sympy import false
from rclpy.action import ActionServer, GoalResponse
import rclpy
import asyncio
from rclpy.node import Node
from std_msgs.msg import String

from tourobo_2026_interfaces.action import BallGet
# pyrefly: ignore [missing-import]
from dyna_interfaces.msg import DynaTarget

import os
import sys


CAN_BUS = can.interface.Bus(bustype="socketcan",
                            channel="can0",
                            asynchronous=True,
                            bitrate=1000000)


class BallGetNode(Node):

    def __init__(self):
        super().__init__('ball_get_node')
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()

        self.declare_parameters(
            namespace='',
            parameters=[
                ('arm_left_id', 20),
                ('arm_right_id', 21),
                ('guard_left_id', 22),
                ('guard_right_id', 23),
                ('shoot_angle_id', 10),
                ('arm_open', 2000),
                ('arm_close', 0),
                ('guard_open', 2000),
                ('guard_close', 0),
                ('shoot_angle_min', 0),
                ('shoot_angle_max', 2000),
                ('shoot_angle_at_gate', 1000),
                ('shoot_push_max', 2000),
                ('shoot_push_min', 0),
                ('shoot_push_intake_gate_ready', 100),
                ('shoot_push_intake_shoot_ready', 200),
                ('shoot_push_shoot_finish', 300),
                ('shoot_push_put_gate_finish', 400),
                ('down_roller_can_id', 0x040),
                ('right_roller_can_id', 0x041),
                ('left_roller_can_id', 0x010),
                ('shoot_roller_1_can_id', 0x011),
                ('shoot_roller_2_can_id', 0x012),
                ('shoot_roller_3_can_id', 0x013),
                ('mini_shoot_can_id', 0x031),
                ('shoot_motor_speed', 1000),
                ('ball_get_down_roller_speed', 1000),
                ('ball_get_up_roller_speed', -1000),
                ('ball_intake_down_roller_speed', 1000),
                ('ball_intake_up_roller_speed', -1000),
                ('ball_put_plate_down_roller_speed', 1000),
                ('ball_put_plate_up_roller_speed', -1000),
                ('wait_time_guard', 1.0),
                ('wait_time_arm', 1.0),
                ('wait_time_shoot_dir', 1.5),
                ('wait_time_roller', 1.0),
                ('wait_time_push', 1.0),
                ('wait_time_get', 1.0),
                ('arm_get_half', 1.0)
            ]
        )

        self.dyna_extpos_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_extpos", 10)
        self.dyna_vel_publisher = self.create_publisher(DynaTarget,
                                                        "/dyna_target_vel", 10)
        self.dyna_pos_publisher = self.create_publisher(DynaTarget,
                                                        "/dyna_target_pos", 10)

        self._action_server = ActionServer(
            self,
            BallGet,
            'ball_get',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        DOWN_ROLLER_CAN_ID = self.get_parameter('down_roller_can_id').value
        RIGHT_ROLLER_CAN_ID = self.get_parameter('right_roller_can_id').value
        LEFT_ROLLER_CAN_ID = self.get_parameter('left_roller_can_id').value

        #dcモーター立ち上げ
        set_pwm_mode(DOWN_ROLLER_CAN_ID, CAN_BUS)
        set_pwm_mode(RIGHT_ROLLER_CAN_ID, CAN_BUS)
        set_pwm_mode(LEFT_ROLLER_CAN_ID, CAN_BUS)

        #初期化
        set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(LEFT_ROLLER_CAN_ID, 0, CAN_BUS)

    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn('現在別の処理を実行中です。新しい指令を拒否します。')
            return GoalResponse.REJECT
        self.get_logger().info('新しい指令を受け付けました。')
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

    # ここがメインの処理じゃぞ
    async def get_ball(self, execute_mode):

        LEFT_ARM_ID = self.get_parameter('arm_left_id').value
        RIGHT_ARM_ID = self.get_parameter('arm_right_id').value
        LEFT_GUARD_ID = self.get_parameter('guard_left_id').value
        RIGHT_GUARD_ID = self.get_parameter('guard_right_id').value
        GUARD_CLOSE = self.get_parameter('guard_close').value
        GATE_CLOSE = self.get_parameter('arm_close').value
        LEFT_ROLLER_CAN_ID = self.get_parameter('left_roller_can_id').value
        RIGHT_ROLLER_CAN_ID = self.get_parameter('right_roller_can_id').value
        DOWN_ROLLER_CAN_ID = self.get_parameter('down_roller_can_id').value
        BALL_GET_DOWN_ROLLER_SPEED = self.get_parameter('ball_get_down_roller_speed').value
        BALL_GET_UP_ROLLER_SPEED = self.get_parameter('ball_get_up_roller_speed').value
        ARM_GET_HALF = self.get_parameter('arm_get_half').value

        WAIT_TIME_GUARD = self.get_parameter('wait_time_guard').value
        WAIT_TIME_ARM = self.get_parameter('wait_time_arm').value
        WAIT_TIME_GET = self.get_parameter('wait_time_get').value

        if execute_mode == 1:  # 左
            # 左脇に保持するために左のガードを閉じる
            self.get_logger().info("左のガードを閉じます")
            self.publish_dyna_pos(LEFT_GUARD_ID, GUARD_CLOSE)
            await asyncio.sleep(WAIT_TIME_GUARD)

            # 上のローラーを回す
            set_goal_pwm(LEFT_ROLLER_CAN_ID, BALL_GET_UP_ROLLER_SPEED,CAN_BUS)
            # 下のローラーを回す
            set_goal_pwm(DOWN_ROLLER_CAN_ID, BALL_GET_DOWN_ROLLER_SPEED,CAN_BUS)

            # ダイナミクセルでゲートを閉じる
            self.get_logger().info("左のゲートを閉じます")
            self.publish_dyna_pos(LEFT_ARM_ID, ARM_GET_HALF)
            await asyncio.sleep(WAIT_TIME_ARM)
            
            #ぼーるが入るのを待つ
            await asyncio.sleep(WAIT_TIME_GET)

            # ろーらーをとめる
            set_goal_pwm(LEFT_ROLLER_CAN_ID, 0, CAN_BUS)
            set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)

            return True

        elif execute_mode == 2:  # 右
            #右脇に保持するために右のガードを閉じる
            self.get_logger().info("右のガードを閉じます")
            self.publish_dyna_pos(RIGHT_GUARD_ID, GUARD_CLOSE)
            await asyncio.sleep(WAIT_TIME_GUARD)

            # 右のローラーを回す
            set_goal_pwm(RIGHT_ROLLER_CAN_ID, BALL_GET_UP_ROLLER_SPEED,CAN_BUS)
            # 下のローラーを回す
            set_goal_pwm(DOWN_ROLLER_CAN_ID, BALL_GET_DOWN_ROLLER_SPEED,CAN_BUS)

            # ダイナミクセルでゲートを閉じる
            self.get_logger().info("右のゲートを閉じます")
            self.publish_dyna_pos(RIGHT_ARM_ID, ARM_GET_HALF)
            await asyncio.sleep(WAIT_TIME_ARM)

            #ボールが入るのを待つ
            await asyncio.sleep(WAIT_TIME_GET)

            # ろーらーをとめる
            set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)
            set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)

            return True

        else:  # エラー
            self.get_logger().info("エラー: dir_numが1または2ではありません")
            return False

    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallGet.Result()
            success = False

            success = await self.get_ball(req.execute_mode)

            if success:
                res.success = True
                # execute_mode 1=左 -> LEFT_CARRY(2), 2=右 -> RIGHT_CARRY(3)
                res.next_state = 2 if req.current_state == 1 else 3
                goal_handle.succeed()
            else:
                goal_handle.abort()

            return res
        finally:
            self.is_executing = False


def main(args=None):
    rclpy.init(args=args)
    node = BallGetNode()
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()

