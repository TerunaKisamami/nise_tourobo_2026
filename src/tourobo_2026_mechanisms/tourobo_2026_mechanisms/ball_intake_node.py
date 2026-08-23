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

CAN_BUS = can.interface.Bus(bustype="socketcan",
                            channel="can0",
                            asynchronous=True,
                            bitrate=1000000)

class BallIntakeNode(Node):

    def __init__(self):
        super().__init__('ball_intake_node')
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
            BallIntake,
            'ball_intake',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        #dcモーター立ち上げ
        set_pwm_mode(DOWN_ROLLER_CAN_ID, CAN_BUS)
        set_pwm_mode(RIGHT_ROLLER_CAN_ID, CAN_BUS)
        set_pwm_mode(LEFT_ROLLER_CAN_ID, CAN_BUS)

        #dcモーター初期化
        set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(LEFT_ROLLER_CAN_ID, 0, CAN_BUS)

        #ロボマスモーター立ち上げ
        set_enc_pos_mode(MINI_SHOOT_CAN_ID, CAN_BUS)

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
    async def get_ball(self, current_state, execute_mode, is_push_max):

        #ダイナミクセルのID

        # ローラー用サーボモーターID

        #発射機構のID

        #左右両方で共通して実行する処理を書く
        #フィードバック無しのため、常に支柱を下げて待機する
        self.get_logger().info("支柱を下げます")
        self.publish_dyna_extpos(SHOOT_ANGLE_ID, SHOOT_ANGLE_MIN)
        time.sleep(WAIT_TIME_SHOOT_ANGLE)

        # current_state は 2: LEFT_CARRY, 3: RIGHT_CARRY
        #left
        if current_state == 2:

            #城門
            if execute_mode == 1:
                self.get_logger().info("左脇にあるボールを内側に取り込みます")

                #押し出し機構を上げる
                if not is_push_max:
                    set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MAX, CAN_BUS)
                    time.sleep(WAIT_TIME_PUSH)

                #右側のガードを下げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

                #左側のガードを上げる
                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_OPEN)
                time.sleep(WAIT_TIME_GUARD)

                #下ローラーを右側へ回転
                set_goal_pwm(DOWN_ROLLER_CAN_ID,BALL_INTAKE_DOWN_ROLLER_SPEED,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(LEFT_ROLLER_CAN_ID,BALL_INTAKE_UP_ROLLER_SPEED,CAN_BUS)

                #左アームを下ろす
                self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_CLOSE)
                time.sleep(WAIT_TIME_ARM)

                #ボールが完全に内側に入るのを待つ
                time.sleep(WAIT_TIME_INTAKE)
                
                #左ガードを下げる
                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

                #ローラーを止める
                set_goal_pwm(DOWN_ROLLER_CAN_ID,0,CAN_BUS)
                set_goal_pwm(LEFT_ROLLER_CAN_ID,0,CAN_BUS)

            #射出
            elif execute_mode == 2:
                self.get_logger().info("左脇にあるボールを内側に取り込みます")

                #押し出し機構を下げる
                self.publish_dyna_extpos(LEFT_ARM_ID,LEFT_ARM_OPEN)
                if is_push_max:
                    set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MIN, CAN_BUS)
                    time.sleep(WAIT_TIME_PUSH)

                #右側のガードを下げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)
                
                #左側のガードを上げる
                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_OPEN)
                time.sleep(WAIT_TIME_GUARD)

                #下ローラーを右側へ回転
                set_goal_pwm(DOWN_ROLLER_CAN_ID,BALL_INTAKE_DOWN_ROLLER_SPEED,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(LEFT_ROLLER_CAN_ID,BALL_INTAKE_UP_ROLLER_SPEED,CAN_BUS)

                #左アームを下ろす
                self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_CLOSE)
                time.sleep(WAIT_TIME_ARM)

                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
                set_goal_pwm(DOWN_ROLLER_CAN_ID,0,CAN_BUS)

                #ボールが完全に内側に入るのを待つ
                time.sleep(WAIT_TIME_INTAKE)

                #左ガードを下げる

                #ローラーを止める
                set_goal_pwm(DOWN_ROLLER_CAN_ID,0,CAN_BUS)
                set_goal_pwm(LEFT_ROLLER_CAN_ID,0,CAN_BUS)
                #time.sleep(WAIT_TIME_GUARD)

        #right
        elif current_state == 3:

            #城門
            if execute_mode == 1:
                self.get_logger().info("右脇にあるボールを内側に取り込みます")

                #押し出し機構を上げる
                if not is_push_max:
                    set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MAX, CAN_BUS)
                    time.sleep(WAIT_TIME_PUSH)

                #左側のガードを下げる
                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

                #右側のガードを上げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_OPEN)
                time.sleep(WAIT_TIME_GUARD)

                #下ローラーを左側へ回転
                set_goal_pwm(DOWN_ROLLER_CAN_ID,BALL_INTAKE_DOWN_ROLLER_SPEED,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(RIGHT_ROLLER_CAN_ID,BALL_INTAKE_UP_ROLLER_SPEED,CAN_BUS)

                #右アームを下ろす
                self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_CLOSE)
                time.sleep(WAIT_TIME_ARM)

                #ボールが完全に内側に入るのを待つ
                time.sleep(WAIT_TIME_INTAKE)

                #右ガードを下げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

                #ローラーを止める
                set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)
                set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)

            #射出
            elif execute_mode == 2:
                self.get_logger().info("右脇にあるボールを内側に取り込みます")

                #押し出し機構を下げる
                if is_push_max:
                    set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MIN, CAN_BUS)
                    time.sleep(WAIT_TIME_PUSH)

                #左側のガードを下げる
                self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

                #右側のガードを上げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_OPEN)
                time.sleep(WAIT_TIME_GUARD)

                #下ローラーを左側へ回転
                set_goal_pwm(DOWN_ROLLER_CAN_ID,BALL_INTAKE_DOWN_ROLLER_SPEED,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(RIGHT_ROLLER_CAN_ID,BALL_INTAKE_UP_ROLLER_SPEED,CAN_BUS)

                #右アームを下ろす
                self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_CLOSE)
                time.sleep(WAIT_TIME_ARM)

                #ボールが完全に内側に入るのを待つ
                time.sleep(WAIT_TIME_INTAKE)

                #右ガードを下げる
                self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
                time.sleep(WAIT_TIME_GUARD)

                #ローラーを止める
                set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)
                set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)

        else:
            self.get_logger().error(
                f"エラー: 想定外の current_state ({current_state}) です。取り込みを中止します。")
            return False

        self.get_logger().info("取り込み完了")
        return True

    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallIntake.Result()
            success = False

            success = await self.get_ball(req.current_state, req.execute_mode, req.is_push_max)

            if success:
                res.success = True
                #城門
                if (req.execute_mode == 1):
                    res.next_state = 4  # INTAKE
                #射出
                elif (req.execute_mode == 2):
                    res.next_state = 5  # INTAKE
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

if __name__ == '__main__':
    main()
