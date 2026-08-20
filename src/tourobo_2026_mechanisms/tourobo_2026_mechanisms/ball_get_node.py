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


class BallGetNode(Node):

    def __init__(self):
        super().__init__('ball_get_node')
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()

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

        #dcモーター立ち上げ
        set_pwm_mode()
        set_pwm_mode()
        set_pwm_mode()
        set_pwm_mode()

        #初期化
        set_goal_pwm()
        set_goal_pwm()
        set_goal_pwm()
        set_goal_pwm()

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

        #これはまだダミーなので後で変える
        THRESHOLD_CLOSED_GATE = 2000  # これより大きければ「閉じている」と判定する
        THRESHOLD_CLOSED_GUARD = 2000  # これより大きければ「閉じている」と判定する

        LEFT_GATE_ID = 20
        RIGHT_GATE_ID = 21
        LEFT_GUARD_ID = 10
        RIGHT_GUARD_ID = 11

        # ローラー用サーボモーターID
        UP_RIGHT_ROLLER_ID = 2
        UP_LEFT_ROLLER_ID = 3
        DOWN_ROLLER_ID = 4

        #発射機構のID
        SHOOT_DIRECTION_ID = 12
        #これより上ならら支柱は上がってる
        ANGLE_SHOOT_UP = 1500

        GUARD_OPEN = 2000
        GUARD_CLOSE = 0
        GATE_OPEN = 2000
        GATE_CLOSE = 0

        #左右両方で共通して実行する処理を書く
        #今ボールを保持しているなら実行しない

        if execute_mode == 1:  # 左
            # わきで保持するために左のガードを閉じる
            self.get_logger().info("左のガードを閉じます")
            self.publish_dyna_pos(LEFT_GUARD_ID, GUARD_CLOSE)
            await asyncio.sleep(1.0)

            # 上のローラーを回すDCモーター

            # 下のローラーを回すDCモーター

            # ダイナミクセルでゲートを閉じる
            self.get_logger().info("左のゲートを閉じます")
            self.publish_dyna_pos(LEFT_GATE_ID, GATE_CLOSE)
            await asyncio.sleep(1.0)

            # 動作終了

            return True

        elif execute_mode == 2:  # 右
            #わきで保持するために右のガードを閉じる
            self.get_logger().info("右のガードを閉じます")
            self.publish_dyna_pos(RIGHT_GUARD_ID, GUARD_CLOSE)
            await asyncio.sleep(1.0)
            # 上のローラーを回す

            # 下のローラーを回す

            # ダイナミクセルでゲートを閉じる
            self.get_logger().info("右のゲートを閉じます")
            self.publish_dyna_pos(RIGHT_GATE_ID, GATE_CLOSE)
            await asyncio.sleep(1.0)

            # 動作終了

        else:  # エラー
            self.get_logger().info("エラー: dir_numが1または2ではありません")

        self.get_logger().info("ボール回収動作が完了しました！")
        return True

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

