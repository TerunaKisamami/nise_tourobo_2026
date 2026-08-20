"""
射出講を上下に動かすだけ
"""
from rclpy.action import ActionServer, GoalResponse
import rclpy
import asyncio
from rclpy.node import Node
from .mechanism_base_node import MechanismBaseNode
from std_msgs.msg import String
import os
import sys

from ah_python_lib.ah_python_can import *
from tourobo_2026_interfaces.action import BallShootAim
from dyna_interfaces.msg import DynaTarget


class BallShootAimNode(MechanismBaseNode):

    def __init__(self):
        super().__init__('ball_shoot_aim_node')
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()





        self._action_server = ActionServer(
            self,
            BallShootAim,
            'ball_shoot_aim',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )


    def publish_dyna_vel(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_vel_publisher.publish(msg)



    #しゃしゅつきこうのじょうげ
    async def aim_ball(self, direction):


        #1: 上げる -1: 下げる 0: 待機
        if direction == 1:
            self.publish_dyna_pos(self.p.shoot_direction_id, self.p.aim_up)
        elif direction == -1:
            self.publish_dyna_pos(self.p.shoot_direction_id, self.p.aim_down)
        elif direction == 0:
            pass

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

            success = await self.aim_ball(req.direction)

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
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()

