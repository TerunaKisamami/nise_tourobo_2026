import can
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


        #射出モーターの立ち上げ
        set_pwm_mode(self.p.shoot_roller_1_can_id, CAN_BUS)
        set_pwm_mode(self.p.shoot_roller_2_can_id, CAN_BUS)
        set_pwm_mode(self.p.shoot_roller_3_can_id, CAN_BUS)

        #射出モーターの初期化
        set_goal_pwm(self.p.shoot_roller_1_can_id, 0, CAN_BUS)
        set_goal_pwm(self.p.shoot_roller_2_can_id, 0, CAN_BUS)
        set_goal_pwm(self.p.shoot_roller_3_can_id, 0, CAN_BUS)

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


        #同時に、3つのモーターを回す
        set_goal_pwm(self.p.shoot_roller_1 ,self.p.shoot_motor_speed, CAN_BUS)
        set_goal_pwm(self.p.shoot_roller_2 ,self.p.shoot_motor_speed, CAN_BUS)
        set_goal_pwm(self.p.shoot_roller_3 ,self.p.shoot_motor_speed, CAN_BUS)
        
        #ろぼますをつかっておしだす
        set_goal_pos(self.p.mini_shoot_id, self.p.shoot_push_shoot_finish, CAN_BUS)
        
        await asyncio.sleep(self.p.wait_time_push)

        #射出モーターを停止
        set_goal_pwm(self.p.shoot_roller_1, 0, CAN_BUS)
        set_goal_pwm(self.p.shoot_roller_2, 0, CAN_BUS)
        set_goal_pwm(self.p.shoot_roller_3, 0, CAN_BUS)

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
                res.next_state = 1  # self.p.not_carry
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

