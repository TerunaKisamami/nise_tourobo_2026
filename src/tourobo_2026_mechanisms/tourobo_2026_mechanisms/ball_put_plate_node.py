"""
ボールを関所に置く処理
"""
from sympy import false
from rclpy.action import ActionServer, GoalResponse
import rclpy
from rclpy.node import Node
from std_msgs.msg import String
from tourobo_2026_interfaces.action import BallPutPlate

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

    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn('現在別の処理を実行中です。新しい指令を拒否します。')
            return GoalResponse.REJECT
        self.get_logger().info('新しい指令を受け付けました。')
        return GoalResponse.ACCEPT
    
    async def put_ball_in_plate(self):
        #左右どちらのゲートが半開きになっているか確認

        #片方だけ半開きになっている状態でなければエラーを出して終了

        #右側だけ半開き(右脇で保持している状態)なら左側から発射
            # 左側のガードを上げる
            # 左側の上のローラーを回す
            # 右側のガードを上げる
            # 右側のゲートを下げる
            # 右側の上のローラーを回す
            # 下のローラーを左向きに回す

            # 左側のゲートを下ろす

        pass
        
        #左側だけ半開き(左脇で保持している状態)なら右側から発射
            # 右側のガードを上げる
            # 右側の上のローラーを回す
            # 左側のガードを上げる
            # 左側の上のローラーを回す
            # 下のローラーを右向きに回す
            
            # 右側のゲートを下ろす

        pass

        #終了処理
        # 左側の上のローラーを止める
        # 右側の上のローラーを止める
        # 下のローラーを止める
        # ガードを落とす

        
    
    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallPutPlate.Result()
            success = False

            success = await self.put_ball_in_plate()
            
            if success:
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
