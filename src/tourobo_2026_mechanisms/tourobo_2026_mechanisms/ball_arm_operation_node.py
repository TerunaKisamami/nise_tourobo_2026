"""
ボールを取り込むアームの開閉をするだけ
ダイナミクセル　アーム開閉
"""
from rclpy.action import ActionServer, GoalResponse
import rclpy
import asyncio
from rclpy.node import Node
from tourobo_2026_mechanisms.constants import *
from std_msgs.msg import String
import os
import sys
import time 

from ah_python_lib.ah_python_can import *
from tourobo_2026_interfaces.action import BallArmOperation
from dyna_interfaces.msg import DynaTarget

class BallArmOperationNode(Node):

    def __init__(self):
        super().__init__('ball_arm_operation_node')
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()

        self.dyna_extpos_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_extpos", 10)
        self.dyna_vel_publisher = self.create_publisher(DynaTarget,
                                                        "/dyna_target_vel", 10)
        self.dyna_pos_publisher = self.create_publisher(DynaTarget,
                                                        "/dyna_target_pos", 10)

        self._action_server = ActionServer(
            self,
            BallArmOperation,
            'ball_arm_operation',
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

    # ここがメインの処理じゃぞ
    async def operate_ball_arm(self, target_arm, is_open):

        DIR_NAME = {1: "左", 2: "右", 0: "エラー"}

        if target_arm not in DIR_NAME:
            self.get_logger().info("エラー: target_armが1(左)または2(右)ではありません")
            return False

        action_name = "開けます" if is_open else "閉じます"
        self.get_logger().info(f"{DIR_NAME[target_arm]}アームを{action_name}")


        # モーターを開閉位置に動かす処理をここに書く
        if target_arm == 1:
            if is_open:
                # 左アームを開く動作
                self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_OPEN)
            else:
                # 左アームを閉じる動作
                self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_CLOSE)
            time.sleep(WAIT_TIME_ARM)
        elif target_arm == 2:
            if is_open:
                # 右アームを開く動作
                self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_OPEN)
            else:
                # 右アームを閉じる動作
                self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_CLOSE)
            time.sleep(WAIT_TIME_ARM)

        return True

    async def execute_callback(self, goal_handle):

        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallArmOperation.Result()
            res.next_state = req.current_state
            res.next_carry = req.carry
            res.next_push_state = req.push_state
            res.next_shoot_angle_state = req.shoot_angle_state
            res.next_is_left_arm_open = req.is_left_arm_open
            res.next_is_right_arm_open = req.is_right_arm_open

            success = False

            success = await self.operate_ball_arm(req.target_arm, req.is_open)

            if success:
                if req.target_arm == 1:
                    res.next_is_left_arm_open = req.is_open
                    if req.current_state == Mechanism_State.SINGLE_CARRY.value and req.carry == BALL_CARRY.LEFT.value:
                        res.next_state = Mechanism_State.NOT_CARRY.value
                        res.next_carry = BALL_CARRY.NOT.value
                elif req.target_arm == 2:
                    res.next_is_right_arm_open = req.is_open
                    if req.current_state == Mechanism_State.SINGLE_CARRY.value and req.carry == BALL_CARRY.RIGHT.value:
                        res.next_state = Mechanism_State.NOT_CARRY.value
                        res.next_carry = BALL_CARRY.NOT.value

                # (Removed overwrite of next_state here)
                res.success = True
                goal_handle.succeed()
            else:
                goal_handle.abort()

            return res
        finally:
            self.is_executing = False

def main(args=None):
    rclpy.init(args=args)
    node = BallArmOperationNode()
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()
