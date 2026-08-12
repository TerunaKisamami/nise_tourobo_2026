from sympy import false
from rclpy.action import ActionServer
import rclpy
from rclpy.node import Node
from std_msgs.msg import String

from tourobo_2026_interfaces.action import BallGet

import os 
import sys

class BallGetNode(Node):
    def __init__(self):
        super().__init__('ball_get_node')
        self.cb_group=rclpy.callback_groups.ReentrantCallbackGroup()
        self._action_server = ActionServer(
            self,
            BallGet,
            'ball_get',
            self.execute_callback,
            callback_group=self.cb_group,
        )
    
    async def get_ball(self):
        #ボールを取得する処理を書く
        pass
    
    async def execute_callback(self, goal_handle):
        req = goal_handle.request
        res = goal_handle.result()
        success = False

        success = await self.get_ball()
        
        if success:
            goal_handle.succeed()
        else:
            goal_handle.abort()
        
        return res
        

def main(args=None):
    rclpy.init(args=args)
    node = BallGetNode()
    executor= rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()
        
if __name__ == '__main__':
    main()