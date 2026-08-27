import can
import osrf_pycommon.process_utils.async_execute_process_asyncio
from rclpy.action import ActionServer, GoalResponse
import rclpy
from rclpy.node import Node
from tourobo_2026_mechanisms.constants import *
from std_msgs.msg import String
import os
import sys
import asyncio
import time

from ah_python_lib.ah_python_can import *
from tourobo_2026_interfaces.action import BallPutGate
from dyna_interfaces.msg import DynaTarget

CAN_BUS = can.interface.Bus(
    bustype="socketcan", channel="can0", asynchronous=True, bitrate=1000000
)


class BallPutGateNode(Node):
    def __init__(self):
        super().__init__("ball_put_gate_node")
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()

        self.dyna_extpos_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_extpos", 10
        )
        self.dyna_vel_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_vel", 10
        )
        self.dyna_pos_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_pos", 10
        )

        self._action_server = ActionServer(
            self,
            BallPutGate,
            "ball_put_gate",
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        # ロボマスモーター立ち上げ
        set_enc_pos_mode(MINI_SHOOT_CAN_ID, CAN_BUS)

    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn("現在別の処理を実行中です。新しい指令を拒否します。")
            return GoalResponse.REJECT
        self.get_logger().info("新しい指令を受け付けました。")
        return GoalResponse.ACCEPT

    # 相対角度
    def publish_dyna_extpos(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_extpos_publisher.publish(msg)

    # 速度制御
    def publish_dyna_vel(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_vel_publisher.publish(msg)

    # 絶対角度
    def publish_dyna_pos(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_pos_publisher.publish(msg)

    # じっさいのどうさぶぶん
    async def put_ball_in_gate(self, current_state, push_state):

        #ga-do wo tojiru
        self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
        self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
 
        # 押し出しを城門側へ
        # ロボマスを使って押し出しを城門側へ
        if push_state != Shoot_Push_State.MIN.value:
            set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MIN, CAN_BUS)
        time.sleep(WAIT_TIME_PUSH)

         #ガードを戻す
        self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_OPEN)
        self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_OPEN)
        #time.sleep(WAIT_TIME_GUARD)

        # 押出機構を上限にあげる
        set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_GATE_HOLD, CAN_BUS)
        time.sleep(WAIT_TIME_PUSH_HALF)

        # 射出角度をあげる
        self.publish_dyna_extpos(SHOOT_ANGLE_ID, SHOOT_ANGLE_MIN)
        time.sleep(WAIT_TIME_SHOOT_ANGLE_PUT_GATE)

        # osidasi kicou wo sageru
        set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MIN ,CAN_BUS)
        time.sleep(WAIT_TIME_PUSH_HALF)

        return True

    async def execute_callback(self, goal_handle):

        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallPutGate.Result()
            res.next_state = req.current_state
            res.next_carry = req.carry
            res.next_push_state = req.push_state
            res.next_shoot_angle_state = req.shoot_angle_state
            res.next_is_left_arm_open = req.is_left_arm_open
            res.next_is_right_arm_open = req.is_right_arm_open

            success = False

            success = await self.put_ball_in_gate(req.current_state, req.push_state)

            if success:
                res.next_push_state = Shoot_Push_State.MIN.value
                res.next_shoot_angle_state = Shoot_Angle_State.MIN.value

                res.success = True
                if req.carry != BALL_CARRY.NOT.value:
                    res.next_state = Mechanism_State.SINGLE_CARRY.value
                    res.next_carry = req.carry
                else:
                    res.next_state = Mechanism_State.NOT_CARRY.value
                    res.next_carry = BALL_CARRY.NOT.value
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


if __name__ == "__main__":
    main()
