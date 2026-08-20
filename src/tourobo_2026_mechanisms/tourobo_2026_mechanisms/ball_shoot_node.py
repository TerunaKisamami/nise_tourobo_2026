from rclpy.action import ActionServer, GoalResponse
import rclpy
from rclpy.node import Node
from .mechanism_base_node import MechanismBaseNode
from std_msgs.msg import String
import os
import sys
import asyncio

from tourobo_2026_interfaces.action import BallShoot
from ah_python_lib.ah_python_can import *
from dyna_interfaces.msg import DynaTarget

CAN_BUS = can.interface.Bus(bustype="socketcan",
                            channel="can0",
                            asynchronous=True,
                            bitrate=1000000)


class BallShootNode(MechanismBaseNode):

    def __init__(self):
        super().__init__('ball_shoot_node')
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()





        self._action_server = ActionServer(
            self,
            BallShoot,
            'ball_shoot',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        SHOOT_ROLLER_1_CAN_ID = self.get_p('shoot_roller_1_can_id')
        SHOOT_ROLLER_2_CAN_ID = self.get_p('shoot_roller_2_can_id')
        SHOOT_ROLLER_3_CAN_ID = self.get_p('shoot_roller_3_can_id')

        #射出モーターの立ち上げ
        set_pwm_mode(SHOOT_ROLLER_1_CAN_ID, CAN_BUS)
        set_pwm_mode(SHOOT_ROLLER_2_CAN_ID, CAN_BUS)
        set_pwm_mode(SHOOT_ROLLER_3_CAN_ID, CAN_BUS)

        #射出モーターの初期化
        set_goal_pwm(SHOOT_ROLLER_1_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(SHOOT_ROLLER_2_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(SHOOT_ROLLER_3_CAN_ID, 0, CAN_BUS)

    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn('現在別の処理を実行中です。新しい指令を拒否します。')
            return GoalResponse.REJECT
        self.get_logger().info('新しい指令を受け付けました。')
        return GoalResponse.ACCEPT


    def publish_dyna_vel(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_vel_publisher.publish(msg)


    async def shoot_ball(self):
        #ボールを発射する処理を書く

        SHOOT_ROLLER_1 = self.get_p('shoot_roller_1_can_id')
        SHOOT_ROLLER_2 = self.get_p('shoot_roller_2_can_id')
        SHOOT_ROLLER_3 = self.get_p('shoot_roller_3_can_id')
        SHOOT_MOTOR_SPEED = self.get_p('shoot_motor_speed')
        SHOOT_PUSH_SHOOT_FINISH = self.get_p('shoot_push_shoot_finish')
        MINI_SHOOT_ID = self.get_p('mini_shoot_can_id')
        WAIT_TIME_PUSH = self.get_p('wait_time_push')

        #同時に、3つのモーターを回す
        set_goal_pwm(SHOOT_ROLLER_1 ,SHOOT_MOTOR_SPEED, CAN_BUS)
        set_goal_pwm(SHOOT_ROLLER_2 ,SHOOT_MOTOR_SPEED, CAN_BUS)
        set_goal_pwm(SHOOT_ROLLER_3 ,SHOOT_MOTOR_SPEED, CAN_BUS)
        
        #ろぼますをつかっておしだす
        set_goal_pos(MINI_SHOOT_ID, SHOOT_PUSH_SHOOT_FINISH, CAN_BUS)
        
        await asyncio.sleep(WAIT_TIME_PUSH)

        #射出モーターを停止
        set_goal_pwm(SHOOT_ROLLER_1, 0, CAN_BUS)
        set_goal_pwm(SHOOT_ROLLER_2, 0, CAN_BUS)
        set_goal_pwm(SHOOT_ROLLER_3, 0, CAN_BUS)

        return True

    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallShoot.Result()
            success = False

            success = await self.shoot_ball()

            if success:
                res.success = True
                res.next_state = 1  # NOT_CARRY
                goal_handle.succeed()
            else:
                goal_handle.abort()

            return res
        finally:
            self.is_executing = False


def main(args=None):
    rclpy.init(args=args)
    node = BallShootNode()
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()

