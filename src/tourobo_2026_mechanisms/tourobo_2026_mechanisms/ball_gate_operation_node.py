"""
ボールを取り込むゲートの開閉をするだけ
"""
from sympy import false
from rclpy.action import ActionServer, GoalResponse
import rclpy
import asyncio
from rclpy.node import Node
from std_msgs.msg import String

from tourobo_2026_interfaces.action import BallGateOperation
# pyrefly: ignore [missing-import]
from dyna_interfaces.msg import DynaTarget, DynaFeedback

import os 
import sys

class BallGateOperationNode(Node):
    def __init__(self):
        super().__init__('ball_gate_operation_node')
        self.is_executing = False
        self.cb_group=rclpy.callback_groups.ReentrantCallbackGroup()
        
        self.dyna_extpos_publisher = self.create_publisher(DynaTarget, "/dyna_target_extpos", 10)
        self.dyna_vel_publisher = self.create_publisher(DynaTarget, "/dyna_target_vel", 10)
        self.dyna_pos_publisher = self.create_publisher(DynaTarget, "/dyna_target_pos", 10)

        # フィードバック受信用
        self.current_dyna_pos = {}
        self.dyna_feedback_sub = self.create_subscription(
            DynaFeedback,
            '/dyna_feedback', # ※CANノード側の仕様に合わせて適宜変更してください
            self.dyna_feedback_callback,
            10,
            callback_group=self.cb_group
        )

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
    
    def dyna_feedback_callback(self, msg):
        # 常に最新の角度データを辞書に保存する
        self.current_dyna_pos[msg.id] = msg.data[0]

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
    async def operate_ball_gate(self,dir_num):
        DIR_NAME={1:"左",2:"右",0:"エラー"}
        self.get_logger().info(f"{DIR_NAME[dir_num]}ゲートを開閉します")

        # 【ダミー設定】閾値やIDは後で調整する
        LEFT_GATE_ID = 20
        RIGHT_GATE_ID = 21

        # 0 ~ 500: 開いている (OPEN)
        # 500 ~ 1500: 半開き (HALF) - 失敗時など
        # 1500 ~ : 閉じている (CLOSED)
        POS_OPEN_THRESHOLD = 500
        POS_HALF_THRESHOLD = 1500

        if dir_num == 1: # 左
            pos = self.current_dyna_pos.get(LEFT_GATE_ID, 0)
            if pos > POS_OPEN_THRESHOLD:
                # 閉じている、または半開き（失敗時）なら開く
                self.get_logger().info("左のゲートが開いていない（閉じている or 半開き）ので、開けます")
            else:
                # 完全に開いているなら閉じる
                self.get_logger().info("左のゲートが開いているので、閉じます")
            pass
        elif dir_num == 2: # 右
            pos = self.current_dyna_pos.get(RIGHT_GATE_ID, 0)
            if pos > POS_OPEN_THRESHOLD:
                # 閉じている、または半開き（失敗時）なら開く
                self.get_logger().info("右のゲートが開いていない（閉じている or 半開き）ので、開けます")
            else:
                # 完全に開いているなら閉じる
                self.get_logger().info("右のゲートが開いているので、閉じます")
            pass
        else: # エラー
            self.get_logger().info("エラー: dir_numが1または2ではありません")    
        
        
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

            success = await self.operate_ball_gate(goal_handle.request.execute_mode)
            
            if success:
                goal_handle.succeed()
            else:
                goal_handle.abort()
            
            return res
        finally:
            self.is_executing = False
        

def main(args=None):
    rclpy.init(args=args)
    node = BallGateOperationNode()
    executor= rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()
        
if __name__ == '__main__':
    main()