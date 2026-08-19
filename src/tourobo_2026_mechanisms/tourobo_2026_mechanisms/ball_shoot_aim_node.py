"""
射出講を上下に動かすだけ
"""
from rclpy.action import ActionServer, GoalResponse
import rclpy
import asyncio
from rclpy.node import Node
from std_msgs.msg import String

from tourobo_2026_interfaces.action import BallShootAim
# pyrefly: ignore [missing-import]
from dyna_interfaces.msg import DynaTarget

import os 
import sys

class BallShootAimNode(Node):
    def __init__(self):
        super().__init__('ball_shoot_aim_node')
        self.is_executing = False
        self.cb_group=rclpy.callback_groups.ReentrantCallbackGroup()
        
        self.dyna_extpos_publisher = self.create_publisher(DynaTarget, "/dyna_target_extpos", 10)
        self.dyna_vel_publisher = self.create_publisher(DynaTarget, "/dyna_target_vel", 10)
        self.dyna_pos_publisher = self.create_publisher(DynaTarget, "/dyna_target_pos", 10)


        self._action_server = ActionServer(
            self,
            BallShootAim,
            'ball_shoot_aim',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )
    

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
    
    async def aim_ball_shoot(self,direction):
        SHOOT_DIRECTION_ID = 12
        AIM_UP = 2000
        AIM_DOWN = 1000

        #1: 上げる -1: 下げる 0: 待機
        if direction == 1:
            self.publish_dyna_pos(SHOOT_DIRECTION_ID, AIM_UP)
        elif direction == -1:
            self.publish_dyna_pos(SHOOT_DIRECTION_ID, AIM_DOWN)
        elif direction == 0:
            pass
        
        await asyncio.sleep(1.0)
        return True
    
    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn('現在別の処理を実行中です。新しい指令を拒否します。')
            return GoalResponse.REJECT
        self.get_logger().info('新しい指令を受け付けました。')
        return GoalResponse.ACCEPT
    
    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallShootAim.Result()
            success = False

            success = await self.aim_ball_shoot(req.direction)
            
            if success:
                goal_handle.succeed()
            else:
                goal_handle.abort()
            
            return res
        finally:
            self.is_executing = False



def main(args=None):
    rclpy.init(args=args)
    node = BallShootAimNode()
    executor= rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()