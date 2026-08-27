import can
from sympy import false
from rclpy.action import ActionServer, GoalResponse
import rclpy
import asyncio
from rclpy.node import Node
from tourobo_2026_mechanisms.constants import *
from std_msgs.msg import String
import time

from tourobo_2026_interfaces.action import BallGet
from dyna_interfaces.msg import DynaTarget
from ah_python_lib.ah_python_can import *
import os
import sys

CAN_BUS = can.interface.Bus(
    bustype="socketcan", channel="can0", asynchronous=True, bitrate=1000000
)


class BallGetNode(Node):
    def __init__(self):
        super().__init__("ball_get_node")
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
            BallGet,
            "ball_get",
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        # dcモーター立ち上げ
        set_pwm_mode(DOWN_ROLLER_CAN_ID, CAN_BUS)
        set_pwm_mode(RIGHT_ROLLER_CAN_ID, CAN_BUS)
        set_pwm_mode(LEFT_ROLLER_CAN_ID, CAN_BUS)

        # 初期化
        set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(LEFT_ROLLER_CAN_ID, 0, CAN_BUS)

    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn("現在別の処理を実行中です。新しい指令を拒否します。")
            return GoalResponse.REJECT
        self.get_logger().info("新しい指令を受け付けました。")
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
    async def get_ball(self, execute_mode):

        # 左右共通して行う処理
        self.get_logger().info(f"execute_mode = {execute_mode} でget_ballが実行された")
        self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_OPEN)
        self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_OPEN)
        time.sleep(WAIT_TIME_GUARD)

        if execute_mode == 1:  # 左
            # 左脇に保持するために左のガードを閉じる
            #            self.get_logger().info("左のガードを閉じます")
            #            self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
            #            time.sleep(WAIT_TIME_GUARD)

            # 上のローラーを回す
            set_goal_pwm(LEFT_ROLLER_CAN_ID, -BALL_GET_UP_ROLLER_SPEED, CAN_BUS)
            # 下のローラーを回す
            set_goal_pwm(DOWN_ROLLER_CAN_ID, BALL_GET_DOWN_ROLLER_SPEED, CAN_BUS)

            # ダイナミクセルでゲートを閉じる
            self.get_logger().info("左のゲートを閉じます")
            self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_GET_HALF)
            #       time.sleep(WAIT_TIME_ARM)

            # ぼーるが入るのを待つ
            time.sleep(WAIT_TIME_GET)

            # ろーらーをとめる
            set_goal_pwm(LEFT_ROLLER_CAN_ID, 0, CAN_BUS)
            set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)

            return True

        elif execute_mode == 2:  # 右
            # 右脇に保持するために右のガードを閉じる
            #            self.get_logger().info("右のガードを閉じます")
            #            self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
            #            time.sleep(WAIT_TIME_GUARD)

            # 右のローラーを回す
            set_goal_pwm(RIGHT_ROLLER_CAN_ID, BALL_GET_UP_ROLLER_SPEED, CAN_BUS)
            # 下のローラーを回す
            set_goal_pwm(DOWN_ROLLER_CAN_ID, -BALL_GET_DOWN_ROLLER_SPEED, CAN_BUS)

            # ダイナミクセルでゲートを閉じる
            self.get_logger().info("右のゲートを閉じます")
            self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_GET_HALF)
            time.sleep(WAIT_TIME_ARM)

            # ボールが入るのを待つ
            #       time.sleep(WAIT_TIME_GET)

            # ろーらーをとめる
            set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)
            set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)

            return True

        else:  # エラー
            self.get_logger().info("エラー: dir_numが1または2ではありません")
            return False

    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallGet.Result()
            res.next_state = req.current_state
            res.next_carry = req.carry
            res.next_push_state = req.push_state
            res.next_shoot_angle_state = req.shoot_angle_state
            res.next_is_left_arm_open = req.is_left_arm_open
            res.next_is_right_arm_open = req.is_right_arm_open

            success = False

            success = await self.get_ball(req.execute_mode)

            if success:
                if req.execute_mode == 1:
                    res.next_is_left_arm_open = False
                elif req.execute_mode == 2:
                    res.next_is_right_arm_open = False

                res.success = True
                # execute_mode 1=左 -> LEFT_CARRY(2), 2=右 -> RIGHT_CARRY(3)
                if req.current_state in [Mechanism_State.INTAKE_GATE.value, Mechanism_State.INTAKE_SHOOT.value]:
                    res.next_state = req.current_state
                else:
                    res.next_state = Mechanism_State.SINGLE_CARRY.value
                res.next_carry = (
                    BALL_CARRY.LEFT.value
                    if req.execute_mode == 1
                    else BALL_CARRY.RIGHT.value
                )
                goal_handle.succeed()
            else:
                goal_handle.abort()

            return res
        finally:
            self.is_executing = False


def main(args=None):
    rclpy.init(args=args)
    node = BallGetNode()
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()
