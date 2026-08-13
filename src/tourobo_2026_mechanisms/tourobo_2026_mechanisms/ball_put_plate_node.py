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
        #ボールをプレートに入れる処理を書く
        pass
    
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
