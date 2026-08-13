from sympy import false
from rclpy.action import ActionServer
import rclpy
from rclpy.node import Node
from std_msgs.msg import String
from tourobo_2026_interfaces.action import BallPutPlate

import os 
import sys

class BallPutPlateNode(Node):
    def __init__(self):
        super().__init__('ball_put_plate_node')
        self.cb_group=rclpy.callback_groups.ReentrantCallbackGroup()
        self._action_server = ActionServer(
            self,
            BallPutPlate,
            'ball_put_plate',
            self.execute_callback,
            callback_group=self.cb_group,
        )
    
    async def put_ball_in_plate(self):
        #ボールをプレートに入れる処理を書く
        pass
    
    async def execute_callback(self, goal_handle):
        req = goal_handle.request
        res = BallPutPlate.Result()
        success = False

        success = await self.put_ball_in_plate()
        
        if success:
            goal_handle.succeed()
        else:
            goal_handle.abort()
        
        return res
        

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
