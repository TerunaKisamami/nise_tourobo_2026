"""
ボールをしろの門に置く動作
"""
import osrf_pycommon.process_utils.async_execute_process_asyncio
from rclpy.action import ActionServer, GoalResponse
import rclpy
from rclpy.node import Node
from .mechanism_base_node import MechanismBaseNode
from std_msgs.msg import String
import os
import sys
import asyncio

from ah_python_lib.ah_python_can import *
from tourobo_2026_interfaces.action import BallPutGate
from dyna_interfaces.msg import DynaTarget

CAN_BUS = can.interface.Bus(bustype="socketcan",
                            channel="can0",
                            asynchronous=True,
                            bitrate=1000000)


class BallPutGateNode(MechanismBaseNode):

    def __init__(self):
        super().__init__('ball_put_gate_node')
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()





        self._action_server = ActionServer(
            self,
            BallPutGate,
            'ball_put_gate',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        #ロボマスモーター立ち上げ
        set_enc_pos_mode(self.p.mini_shoot_can_id, CAN_BUS)

    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn('現在別の処理を実行中です。新しい指令を拒否します。')
            return GoalResponse.REJECT
        self.get_logger().info('新しい指令を受け付けました。')
        return GoalResponse.ACCEPT

    #相対角度

    #速度制御
    def publish_dyna_vel(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_vel_publisher.publish(msg)

    #絶対角度


    #じっさいのどうさぶぶん
    async def put_ball_in_gate(self, current_state):

        #射出角度をさげる
        self.publish_dyna_extpos(self.p.shoot_angle_id, self.p.shoot_angle_at_gate)
        await asyncio.sleep(self.p.wait_time_shoot_dir_put_gate)

        #押し出しを城門側へ
        # ロボマスを使って押し出しを城門側へ
        set_goal_pos(self.p.mini_shoot_can_id, self.p.shoot_push_gate_finish, CAN_BUS)
        await asyncio.sleep(self.p.wait_time_push)

        return True

    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallPutGate.Result()
            success = False

            success = await self.put_ball_in_gate(req.current_state)

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
    node = BallPutGateNode()
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()

