"""
どういう動きをするか↓

"""
from sympy import false
from rclpy.action import ActionServer, GoalResponse
import rclpy
import asyncio
from rclpy.node import Node
from .mechanism_base_node import MechanismBaseNode
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


class BallGetNode(MechanismBaseNode):

    def __init__(self):
        super().__init__('ball_get_node')
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()





        self._action_server = ActionServer(
            self,
            BallGet,
            'ball_get',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        DOWN_ROLLER_CAN_ID = self.get_p('down_roller_can_id')
        RIGHT_ROLLER_CAN_ID = self.get_p('right_roller_can_id')
        LEFT_ROLLER_CAN_ID = self.get_p('left_roller_can_id')

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


    def publish_dyna_vel(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_vel_publisher.publish(msg)


    # ここがメインの処理じゃぞ
    async def get_ball(self, execute_mode):

        LEFT_ARM_ID = self.get_p('arm_left_id')
        RIGHT_ARM_ID = self.get_p('arm_right_id')
        LEFT_GUARD_ID = self.get_p('guard_left_id')
        RIGHT_GUARD_ID = self.get_p('guard_right_id')
        LEFT_ROLLER_CAN_ID = self.get_p('left_roller_can_id')
        RIGHT_ROLLER_CAN_ID = self.get_p('right_roller_can_id')
        DOWN_ROLLER_CAN_ID = self.get_p('down_roller_can_id')
        BALL_GET_DOWN_ROLLER_SPEED = self.get_p('ball_get_down_roller_speed')
        BALL_GET_UP_ROLLER_SPEED = self.get_p('ball_get_up_roller_speed')

        WAIT_TIME_GUARD = self.get_p('wait_time_guard')

        LEFT_ARM_OPEN = self.get_p('arm_left_open')
        RIGHT_ARM_OPEN = self.get_p('arm_right_open')
        LEFT_ARM_CLOSE = self.get_p('arm_left_close')
        RIGHT_ARM_CLOSE = self.get_p('arm_right_close')
        LEFT_ARM_GET_HALF = self.get_p('arm_left_get_half')
        RIGHT_ARM_GET_HALF = self.get_p('arm_right_get_half')
        LEFT_GUARD_OPEN = self.get_p('guard_left_open')
        RIGHT_GUARD_OPEN = self.get_p('guard_right_open')
        LEFT_GUARD_CLOSE = self.get_p('guard_left_close')
        RIGHT_GUARD_CLOSE = self.get_p('guard_right_close')
        WAIT_TIME_ARM = self.get_p('wait_time_arm')
        WAIT_TIME_GET = self.get_p('wait_time_get')

        if execute_mode == 1:  # 左
            # 左脇に保持するために左のガードを閉じる
            self.get_logger().info("左のガードを閉じます")
            self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
            await asyncio.sleep(WAIT_TIME_GUARD)

            # 上のローラーを回す
            set_goal_pwm(LEFT_ROLLER_CAN_ID, BALL_GET_UP_ROLLER_SPEED,CAN_BUS)
            # 下のローラーを回す
            set_goal_pwm(DOWN_ROLLER_CAN_ID, BALL_GET_DOWN_ROLLER_SPEED,CAN_BUS)

            # ダイナミクセルでゲートを閉じる
            self.get_logger().info("左のゲートを閉じます")
            self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_GET_HALF)
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
            self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
            await asyncio.sleep(WAIT_TIME_GUARD)

            # 右のローラーを回す
            set_goal_pwm(RIGHT_ROLLER_CAN_ID, BALL_GET_UP_ROLLER_SPEED,CAN_BUS)
            # 下のローラーを回す
            set_goal_pwm(DOWN_ROLLER_CAN_ID, BALL_GET_DOWN_ROLLER_SPEED,CAN_BUS)

            # ダイナミクセルでゲートを閉じる
            self.get_logger().info("右のゲートを閉じます")
            self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_GET_HALF)
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

