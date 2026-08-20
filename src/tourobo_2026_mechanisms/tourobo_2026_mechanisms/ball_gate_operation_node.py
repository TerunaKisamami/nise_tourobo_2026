"""
ボールを取り込むゲートの開閉をするだけ
ダイナミクセル　ゲート開閉
"""
from rclpy.action import ActionServer, GoalResponse
import rclpy
import asyncio
from rclpy.node import Node
from std_msgs.msg import String
import os
import sys

from ah_python_lib.ah_python_can import *
from tourobo_2026_interfaces.action import BallGateOperation
from dyna_interfaces.msg import DynaTarget


class BallGateOperationNode(Node):

    def __init__(self):
        super().__init__('ball_gate_operation_node')
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
            BallGateOperation,
            'ball_gate_operation',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

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
    async def operate_ball_gate(self, target_gate, is_open):
        DIR_NAME = {1: "左", 2: "右", 0: "エラー"}
        if target_gate not in DIR_NAME:
            self.get_logger().info("エラー: target_gateが1(左)または2(右)ではありません")
            return False

        action_name = "開けます" if is_open else "閉じます"
        self.get_logger().info(f"{DIR_NAME[target_gate]}ゲートを{action_name}")

        # 【ダミー設定】閾値やIDは後で調整する
        LEFT_GATE_ID = 20
        RIGHT_GATE_ID = 21
        GATE_OPEN = 2000
        GATE_CLOSE = 0

        # モーターを開閉位置に動かす処理をここに書く
        if target_gate == 1:
            if is_open:
                # 左ゲートを開く動作
                self.publish_dyna_pos(LEFT_GATE_ID, GATE_OPEN)
            else:
                # 左ゲートを閉じる動作
                self.publish_dyna_pos(LEFT_GATE_ID, GATE_CLOSE)
            await asyncio.sleep(1.0)
        elif target_gate == 2:
            if is_open:
                # 右ゲートを開く動作
                self.publish_dyna_pos(RIGHT_GATE_ID, GATE_OPEN)
            else:
                # 右ゲートを閉じる動作
                self.publish_dyna_pos(RIGHT_GATE_ID, GATE_CLOSE)
            await asyncio.sleep(1.0)
        """
        # 例: アームを下げる (ID: 10, Pos: 2000)
        self.get_logger().info("アームを下ろします")
        self.publish_dyna_pos(10, 2000)
        await asyncio.sleep(2.0) # 2秒待機 (time.sleepは使わないこと！)

        # 例: ローラーを回す (ID: 11, Vel: 100)
        self.get_logger().info("ローラーを回転させます")
        self.publish_dyna_vel(11, 100)
        await asyncio.sleep(1.0)

        # 例: アームを上げる (ID: 10, Pos: 0)
        self.get_logger().info("アームを上げます")
        self.publish_dyna_pos(10, 0)
        await asyncio.sleep(2.0)

        # 例: ローラーを止める (ID: 11, Vel: 0)
        self.get_logger().info("ローラーを停止します")
        self.publish_dyna_vel(11, 0)
        """

        return True

    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallGateOperation.Result()
            success = False

            success = await self.operate_ball_gate(req.target_gate, req.is_open)

            if success:
                res.next_state = req.current_state  # ゲート開閉はメインの論理状態を変えない
                res.success = True
                goal_handle.succeed()
            else:
                goal_handle.abort()

            return res
        finally:
            self.is_executing = False


def main(args=None):
    rclpy.init(args=args)
    node = BallGateOperationNode()
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
