import can
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
from tourobo_2026_interfaces.action import BallIntake
from dyna_interfaces.msg import DynaTarget

CAN_BUS = can.interface.Bus(
    bustype="socketcan", channel="can0", asynchronous=True, bitrate=1000000
)


class BallIntakeNode(Node):
    def __init__(self):
        super().__init__("ball_intake_node")
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
            BallIntake,
            "ball_intake",
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        # dcモーター立ち上げ
        set_pwm_mode(DOWN_ROLLER_CAN_ID, CAN_BUS)
        set_pwm_mode(RIGHT_ROLLER_CAN_ID, CAN_BUS)
        set_pwm_mode(LEFT_ROLLER_CAN_ID, CAN_BUS)

        # dcモーター初期化
        set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(LEFT_ROLLER_CAN_ID, 0, CAN_BUS)

        # ロボマスモーター立ち上げ
        set_enc_pos_mode(MINI_SHOOT_CAN_ID, CAN_BUS)

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
    async def intake_ball(
        self, carry, execute_mode, push_state, shoot_angle_state
    ):
        # 左右両方で共通して実行する処理を書く
        # フィードバック無しのため、常に支柱を下げて待機する
        if shoot_angle_state != Shoot_Angle_State.MIN.value:
            self.get_logger().info("支柱を下げます")
            self.publish_dyna_extpos(SHOOT_ANGLE_ID, SHOOT_ANGLE_MIN)
            time.sleep(WAIT_TIME_SHOOT_ANGLE)

        # current_state は 2: LEFT_CARRY, 3: RIGHT_CARRY
        # 城門
        if execute_mode == 1:
            # 左右共通して行う前処理
            # 押し出し機構を上げる
            if push_state != Shoot_Push_State.MAX.value:
                set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MAX, CAN_BUS)
                time.sleep(WAIT_TIME_PUSH)

            # left
            if carry == 1:
                self.get_logger().info("左脇にあるボールをintake(城門)します")

                # 右側のガードを下げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

                # 左側のガードを上げる
                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_OPEN)
                time.sleep(WAIT_TIME_GUARD)

                # 下ローラーを右側へ回転
                set_goal_pwm(DOWN_ROLLER_CAN_ID, BALL_INTAKE_DOWN_ROLLER_SPEED, CAN_BUS)

                # 上ローラーを回転
                set_goal_pwm(LEFT_ROLLER_CAN_ID, -BALL_INTAKE_UP_ROLLER_SPEED, CAN_BUS)

                # 左アームを下ろす
                self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_CLOSE)
                time.sleep(WAIT_TIME_ARM)

                # ボールが完全に内側に入るのを待つ
                time.sleep(WAIT_TIME_INTAKE)

                # 左ガードを下げる
                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

            # right
            elif carry == 2:
                self.get_logger().info("右脇にあるボールをintake(城門)します")

                # 左側のガードを下げる
                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

                # 右側のガードを上げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_OPEN)
                time.sleep(WAIT_TIME_GUARD)

                # 下ローラーを左側へ回転
                set_goal_pwm(DOWN_ROLLER_CAN_ID, -BALL_INTAKE_DOWN_ROLLER_SPEED, CAN_BUS)

                # 上ローラーを回転
                set_goal_pwm(RIGHT_ROLLER_CAN_ID, BALL_INTAKE_UP_ROLLER_SPEED, CAN_BUS)

                # 右アームを下ろす
                self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_CLOSE)
                time.sleep(WAIT_TIME_ARM)

                # ボールが完全に内側に入るのを待つ
                time.sleep(WAIT_TIME_INTAKE)

                # 右ガードを下げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

            # 左右共通して行う後処理
            # ローラーを止める
            set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)
            set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)
            set_goal_pwm(LEFT_ROLLER_CAN_ID, 0, CAN_BUS)

            # ボールが引っかからないように押出機構で支える
            set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_GATE_HOLD, CAN_BUS)
            time.sleep(WAIT_TIME_PUSH_HALF)

            #射出角度をさげる
            self.publish_dyna_extpos(SHOOT_ANGLE_ID, SHOOT_ANGLE_AT_GATE)
            time.sleep(WAIT_TIME_SHOOT_ANGLE_PUT_GATE)


        # 射出
        elif execute_mode == 2:
            # 左右共通して行う前処理
            # 押し出し機構を下げる
            if push_state != Shoot_Push_State.MIN.value:
                set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MIN, CAN_BUS)
                time.sleep(WAIT_TIME_PUSH)


            # left
            if carry == 1:
                self.get_logger().info("左脇にあるボールを内側に取り込みます")

                # 右側のガードを下げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

                # 左側のガードを上げる
                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_OPEN)
                time.sleep(WAIT_TIME_GUARD)

                # 下ローラーを右側へ回転
                set_goal_pwm(DOWN_ROLLER_CAN_ID, BALL_INTAKE_DOWN_ROLLER_SPEED, CAN_BUS)

                # 上ローラーを回転
                set_goal_pwm(LEFT_ROLLER_CAN_ID, -BALL_INTAKE_UP_ROLLER_SPEED, CAN_BUS)

                # 左アームを下ろす
                self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_CLOSE)
                time.sleep(WAIT_TIME_ARM)

                # ボールが完全に内側に入るのを待つ
                time.sleep(WAIT_TIME_INTAKE)

                # 左ガードを下げる
                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)

                set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)


                # time.sleep(WAIT_TIME_GUARD)

            # right
            if carry == 2:
                self.get_logger().info("右脇にあるボールを内側に取り込みます")

                # 左側のガードを下げる
                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

                # 右側のガードを上げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_OPEN)
                time.sleep(WAIT_TIME_GUARD)

                # 下ローラーを左側へ回転
                set_goal_pwm(DOWN_ROLLER_CAN_ID, -BALL_INTAKE_DOWN_ROLLER_SPEED, CAN_BUS)

                # 上ローラーを回転
                set_goal_pwm(RIGHT_ROLLER_CAN_ID, BALL_INTAKE_UP_ROLLER_SPEED, CAN_BUS)

                # 右アームを下ろす
                self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_CLOSE)
                time.sleep(WAIT_TIME_ARM)

                # ボールが完全に内側に入るのを待つ
                time.sleep(WAIT_TIME_INTAKE)

                # 右ガードを下げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

            # 左右共通して行う後処理
            # ローラーを止める
            set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)
            set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)
            set_goal_pwm(LEFT_ROLLER_CAN_ID, 0, CAN_BUS)

            # 射出機構の直前までボールをセットしておく
            set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_LOADING,CAN_BUS)
            time.sleep(WAIT_TIME_PUSH_HALF)

        else:
            self.get_logger().error(
                f"エラー: 想定外の current_state ({current_state}) です。取り込みを中止します。"
            )
            return False

        self.get_logger().info("取り込み完了")
        return True

    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallIntake.Result()
            res.next_state = req.current_state
            res.next_carry = req.carry
            res.next_push_state = req.push_state
            res.next_shoot_angle_state = req.shoot_angle_state
            res.next_is_left_arm_open = req.is_left_arm_open
            res.next_is_right_arm_open = req.is_right_arm_open

            success = False

            success = await self.intake_ball(
                req.carry,
                req.execute_mode,
                req.push_state,
                req.shoot_angle_state,
            )

            if success:
                if req.execute_mode == 1:
                    res.next_push_state = Shoot_Push_State.GATE_HOLD.value
                elif req.execute_mode == 2:
                    res.next_push_state = Shoot_Push_State.LOADING.value

                res.success = True
                # 城門
                if req.execute_mode == 1:
                    res.next_state = Mechanism_State.INTAKE_GATE.value
                # 射出
                elif req.execute_mode == 2:
                    res.next_state = Mechanism_State.INTAKE_SHOOT.value
                
                res.next_carry = BALL_CARRY.NOT.value
                goal_handle.succeed()
            else:
                goal_handle.abort()

            return res
        finally:
            self.is_executing = False


def main(args=None):
    rclpy.init(args=args)
    node = BallIntakeNode()
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()
