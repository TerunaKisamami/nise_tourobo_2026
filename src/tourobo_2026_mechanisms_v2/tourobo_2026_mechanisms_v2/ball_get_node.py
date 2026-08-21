import can
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


        #dcモーター立ち上げ
        set_pwm_mode(self.p.down_roller_can_id, CAN_BUS)
        set_pwm_mode(self.p.right_roller_can_id, CAN_BUS)
        set_pwm_mode(self.p.left_roller_can_id, CAN_BUS)

        #初期化
        set_goal_pwm(self.p.down_roller_can_id, 0, CAN_BUS)
        set_goal_pwm(self.p.right_roller_can_id, 0, CAN_BUS)
        set_goal_pwm(self.p.left_roller_can_id, 0, CAN_BUS)

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




        if execute_mode == 1:  # 左
            # 左脇に保持するために左のガードを閉じる
            self.get_logger().info("左のガードを閉じます")
            self.publish_dyna_extpos(self.p.left_guard_id, self.p.guard_left_close)
            await asyncio.sleep(self.p.wait_time_guard)

            # 上のローラーを回す
            set_goal_pwm(self.p.left_roller_can_id, self.p.ball_get_up_roller_speed,CAN_BUS)
            # 下のローラーを回す
            set_goal_pwm(self.p.down_roller_can_id, self.p.ball_get_down_roller_speed,CAN_BUS)

            # ダイナミクセルでゲートを閉じる
            self.get_logger().info("左のゲートを閉じます")
            self.publish_dyna_extpos(self.p.left_arm_id, self.p.arm_left_get_half)
            await asyncio.sleep(self.p.wait_time_arm)
            
            #ぼーるが入るのを待つ
            await asyncio.sleep(self.p.wait_time_get)

            # ろーらーをとめる
            set_goal_pwm(self.p.left_roller_can_id, 0, CAN_BUS)
            set_goal_pwm(self.p.down_roller_can_id, 0, CAN_BUS)

            return True

        elif execute_mode == 2:  # 右
            #右脇に保持するために右のガードを閉じる
            self.get_logger().info("右のガードを閉じます")
            self.publish_dyna_extpos(self.p.right_guard_id, self.p.guard_right_close)
            await asyncio.sleep(self.p.wait_time_guard)

            # 右のローラーを回す
            set_goal_pwm(self.p.right_roller_can_id, self.p.ball_get_up_roller_speed,CAN_BUS)
            # 下のローラーを回す
            set_goal_pwm(self.p.down_roller_can_id, self.p.ball_get_down_roller_speed,CAN_BUS)

            # ダイナミクセルでゲートを閉じる
            self.get_logger().info("右のゲートを閉じます")
            self.publish_dyna_extpos(self.p.right_arm_id, self.p.arm_right_get_half)
            await asyncio.sleep(self.p.wait_time_arm)

            #ボールが入るのを待つ
            await asyncio.sleep(self.p.wait_time_get)

            # ろーらーをとめる
            set_goal_pwm(self.p.right_roller_can_id, 0, CAN_BUS)
            set_goal_pwm(self.p.down_roller_can_id, 0, CAN_BUS)

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
                # execute_mode 1=左 -> self.p.left_carry(2), 2=右 -> self.p.right_carry(3)
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

