"""
ボールを関所に置く処理
"""
from sympy import false
from rclpy.action import ActionServer, GoalResponse
import rclpy
from rclpy.node import Node
from std_msgs.msg import String
from tourobo_2026_interfaces.action import BallPutPlate
from dyna_interfaces.msg import DynaTarget

import asyncio

import os 
import sys

class BallPutPlateNode(Node):
    def __init__(self):
        super().__init__('ball_put_plate_node')
        self.is_executing = False
        self.cb_group=rclpy.callback_groups.ReentrantCallbackGroup()
        self._action_server = ActionServer(
            self,
            BallPutPlate,
            'ball_put_plate',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        self.dyna_extpos_publisher = self.create_publisher(DynaTarget, "/dyna_target_extpos", 10)
        self.dyna_vel_publisher = self.create_publisher(DynaTarget, "/dyna_target_vel", 10)
        self.dyna_pos_publisher = self.create_publisher(DynaTarget, "/dyna_target_pos", 10)

    def publish_dyna_pos(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_pos_publisher.publish(msg)

    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn('現在別の処理を実行中です。新しい指令を拒否します。')
            return GoalResponse.REJECT
        self.get_logger().info('新しい指令を受け付けました。')
        return GoalResponse.ACCEPT
    
    async def put_ball_in_plate(self, current_state):
        # 仮定数追加
        LEFT_GATE_ID = 20
        RIGHT_GATE_ID = 21
        LEFT_GUARD_ID = 10
        RIGHT_GUARD_ID = 11
        GUARD_OPEN = 2000
        GUARD_CLOSE = 0
        GATE_OPEN = 2000
        GATE_CLOSE = 0

        # current_state: 2=LEFT_CARRY, 3=RIGHT_CARRY
        if current_state == 3:
            self.get_logger().info("右脇で保持している状態から、左側へボールを関所に置きます")
            #右側だけ半開き(右脇で保持している状態)なら左側から発射
            # 左側のゲートを上げる
            self.publish_dyna_pos(LEFT_GATE_ID, GATE_OPEN)
            # 左側のガードを上げる
            self.publish_dyna_pos(LEFT_GUARD_ID, GUARD_CLOSE)
            await asyncio.sleep(1.0)
            # 左側の上のローラーを回す
            # 右側のガードを上げる
            self.publish_dyna_pos(RIGHT_GUARD_ID, GUARD_CLOSE)
            # 右側のゲートを下げる
            self.publish_dyna_pos(RIGHT_GATE_ID, GATE_CLOSE)
            await asyncio.sleep(1.0)
            # 右側の上のローラーを回す
            # 下のローラーを左向きに回す

            # 左側のゲートを下ろす
            self.publish_dyna_pos(LEFT_GATE_ID, GATE_CLOSE)
            await asyncio.sleep(1.0)

        elif current_state == 2:
            self.get_logger().info("左脇で保持している状態から、右側へボールを関所に置きます")
            #左側だけ半開き(左脇で保持している状態)なら右側から発射
            # 右側のゲートを上げる
            self.publish_dyna_pos(RIGHT_GATE_ID, GATE_OPEN)
            # 右側のガードを上げる
            self.publish_dyna_pos(RIGHT_GUARD_ID, GUARD_CLOSE)
            await asyncio.sleep(1.0)
            # 右側の上のローラーを回す
            # 左側のガードを上げる
            self.publish_dyna_pos(LEFT_GUARD_ID, GUARD_CLOSE)
            # 左側の上のローラーを回す
            # 下のローラーを右向きに回す
            
            # 右側のゲートを下ろす
            self.publish_dyna_pos(RIGHT_GATE_ID, GATE_CLOSE)
            await asyncio.sleep(1.0)

        else:
            self.get_logger().error(f"エラー: 想定外の current_state ({current_state}) です。")
            return False

        #終了処理
        # 左側の上のローラーを止める
        # 右側の上のローラーを止める
        # 下のローラーを止める
        # ガードを落とす
        
        self.get_logger().info("関所への配置完了")
        return True
    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallPutPlate.Result()
            success = False

            success = await self.put_ball_in_plate(req.current_state)
            
            if success:
                res.success = True
                res.next_state = 1 # NOT_CARRY
                goal_handle.succeed()
            else:
                goal_handle.abort()
            
            return res
        finally:
            self.is_executing = False
        

def main(args=None):
    rclpy.init(args=args)
    node = BallPutPlateNode()
    executor= rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()
        
if __name__ == '__main__':
    main()
