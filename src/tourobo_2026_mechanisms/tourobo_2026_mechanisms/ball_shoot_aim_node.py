"""
射出講を上下に動かすだけ
"""
from rclpy.action import ActionServer, GoalResponse
import rclpy
import asyncio
from rclpy.node import Node
from std_msgs.msg import String
import os
import sys

from ah_python_lib.ah_python_can import *
from tourobo_2026_interfaces.action import BallShootAim
from dyna_interfaces.msg import DynaTarget


class BallShootAimNode(Node):

    def __init__(self):
        super().__init__('ball_shoot_aim_node')
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()

        self.declare_parameters(
            namespace='',
            parameters=[
                ('arm_left_id', 20),
                ('arm_right_id', 21),
                ('guard_left_id', 22),
                ('guard_right_id', 23),
                ('shoot_angle_id', 10),

                ('arm_left_open', 2000),
                ('arm_right_open', 2000),
                ('arm_left_close', 0),
                ('arm_right_close', 0),
                ('arm_left_get_half', 1000),
                ('arm_right_get_half', 1000),
                ('guard_left_open', 2000),
                ('guard_right_open', 2000),
                ('guard_left_close', 0),
                ('guard_right_close', 0),
                ('arm_left_open', 2000),
                ('arm_right_open', 2000),
                ('arm_left_close', 0),
                ('arm_right_close', 0),
                ('guard_left_open', 2000),
                ('guard_right_open', 2000),
                ('guard_left_close', 0),
                ('guard_right_close', 0),
                ('shoot_angle_min', 0),
                ('shoot_angle_max', 2000),
                ('shoot_angle_at_gate', 1000),
                ('shoot_push_max', 2000),
                ('shoot_push_min', 0),
                ('shoot_push_intake_gate_ready', 100),
                ('shoot_push_intake_shoot_ready', 200),
                ('shoot_push_shoot_finish', 300),
                ('shoot_push_put_gate_finish', 400),
                ('down_roller_can_id', 0x040),
                ('right_roller_can_id', 0x041),
                ('left_roller_can_id', 0x010),
                ('shoot_roller_1_can_id', 0x011),
                ('shoot_roller_2_can_id', 0x012),
                ('shoot_roller_3_can_id', 0x013),
                ('mini_shoot_can_id', 0x031),
                ('shoot_motor_speed', 1000),
                ('ball_get_down_roller_speed', 1000),
                ('ball_get_up_roller_speed', -1000),
                ('ball_intake_down_roller_speed', 1000),
                ('ball_intake_up_roller_speed', -1000),
                ('ball_put_plate_down_roller_speed', 1000),
                ('ball_put_plate_up_roller_speed', -1000),
                ('wait_time_guard', 1.0),
                ('wait_time_arm', 1.0),
                ('wait_time_shoot_dir', 1.5),
                ('wait_time_roller', 1.0),
                ('wait_time_push', 1.0),
                ('wait_time_get', 1.0),
                ('wait_time_intake', 1.0),
                ('wait_time_put_plate', 1.0)
            ]
        )

        self.dyna_extpos_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_extpos", 10)
        self.dyna_vel_publisher = self.create_publisher(DynaTarget,
                                                        "/dyna_target_vel", 10)
        self.dyna_pos_publisher = self.create_publisher(DynaTarget,
                                                        "/dyna_target_pos", 10)

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


    #しゃしゅつきこうのじょうげ
    async def aim_ball(self, direction):
        SHOOT_DIRECTION_ID = self.get_parameter('shoot_angle_id').value

        LEFT_ARM_OPEN = self.get_parameter('arm_left_open').value
        RIGHT_ARM_OPEN = self.get_parameter('arm_right_open').value
        LEFT_ARM_CLOSE = self.get_parameter('arm_left_close').value
        RIGHT_ARM_CLOSE = self.get_parameter('arm_right_close').value
        LEFT_ARM_GET_HALF = self.get_parameter('arm_left_get_half').value
        RIGHT_ARM_GET_HALF = self.get_parameter('arm_right_get_half').value
        LEFT_GUARD_OPEN = self.get_parameter('guard_left_open').value
        RIGHT_GUARD_OPEN = self.get_parameter('guard_right_open').value
        LEFT_GUARD_CLOSE = self.get_parameter('guard_left_close').value
        RIGHT_GUARD_CLOSE = self.get_parameter('guard_right_close').value
        AIM_UP = self.get_parameter('shoot_angle_max').value
        AIM_DOWN = self.get_parameter('shoot_angle_min').value
        WAIT_TIME_SHOOT_DIR = self.get_parameter('wait_time_shoot_dir').value

        #1: 上げる -1: 下げる 0: 待機
        if direction == 1:
            self.publish_dyna_pos(SHOOT_DIRECTION_ID, AIM_UP)
        elif direction == -1:
            self.publish_dyna_pos(SHOOT_DIRECTION_ID, AIM_DOWN)
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

