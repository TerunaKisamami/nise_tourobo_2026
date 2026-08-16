"""
わきにかかえたボールを内側へ取り込む
"""

from sympy import false
from rclpy.action import ActionServer, GoalResponse
import rclpy
from rclpy.node import Node
from std_msgs.msg import String
from tourobo_2026_interfaces.action import BallIntake
# pyrefly: ignore [missing-import]
from dyna_interfaces.msg import DynaTarget, DynaFeedback

import os 
import sys

class BallIntakeNode(Node):
    def __init__(self):
        super().__init__('ball_intake_node')
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
            BallIntake,
            'ball_intake',
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
        # msg.data[0] に位置情報が入っていると仮定
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
    async def get_ball(self):
        #これはまだダミーなので後で変える
        THRESHOLD_CLOSED_GATE = 2000 # これより大きければ「閉じている」と判定する
        THRESHOLD_CLOSED_GUARD = 2000 # これより大きければ「閉じている」と判定する

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
        

        #左右両方で共通して実行する処理を書く
        #今ボールを内側に保持しているなら実行しない

        #左右どちらの脇にボールを挟んでいるか判断する

        #片方だけ半開きでない状態だったらエラー
    
        #左脇にボールを挟んでいる場合
            #右側のガードを下げる
            #左側のガードを上げる
            #下ローラーを右側へ回転
            #左ゲートを下ろす


        #右脇にボールを挟んでいる場合
            #左側のガードを下げる
            #右側のガードを上げる
            #下ローラーを左側へ回転
            #右ゲートを下ろす
            
        #エラー処理

        #終了処理
        #ローラーを止める

        

    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallIntake.Result()
            success = False

            success = await self.get_ball()
            
            if success:
                goal_handle.succeed()
            else:
                goal_handle.abort()
            
            return res
        finally:
            self.is_executing = False
        

def main(args=None):
    rclpy.init(args=args)
    node = BallIntakeNode()
    executor= rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()
        
if __name__ == '__main__':
    main()