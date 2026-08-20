"""
わきにかかえたボールを内側へ取り込む
"""

from rclpy.action import ActionServer, GoalResponse
import rclpy
from rclpy.node import Node
from std_msgs.msg import String
import os
import sys
import asyncio

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

        self.declare_parameters(
            namespace='',
            parameters=[
                ('arm_left_id', 20),
                ('arm_right_id', 21),
                ('guard_left_id', 22),
                ('guard_right_id', 23),
                ('shoot_angle_id', 10),
                ('arm_open', 2000),
                ('arm_close', 0),
                ('guard_open', 2000),
                ('guard_close', 0),
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
            BallIntake,
            'ball_intake',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        DOWN_ROLLER_CAN_ID = self.get_parameter('down_roller_can_id').value
        RIGHT_ROLLER_CAN_ID = self.get_parameter('right_roller_can_id').value
        LEFT_ROLLER_CAN_ID = self.get_parameter('left_roller_can_id').value
        MINI_SHOOT_CAN_ID = self.get_parameter('mini_shoot_can_id').value

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
    async def get_ball(self, current_state, execute_mode):

        #ダイナミクセルのID
        LEFT_ARM_ID = self.get_parameter('arm_left_id').value
        RIGHT_ARM_ID = self.get_parameter('arm_right_id').value
        LEFT_GUARD_ID = self.get_parameter('guard_left_id').value
        RIGHT_GUARD_ID = self.get_parameter('guard_right_id').value

        # ローラー用サーボモーターID
        UP_RIGHT_ROLLER_ID = self.get_parameter('right_roller_can_id').value
        UP_LEFT_ROLLER_ID = self.get_parameter('left_roller_can_id').value
        DOWN_ROLLER_ID = self.get_parameter('down_roller_can_id').value

        #発射機構のID
        SHOOT_DIRECTION_ID = self.get_parameter('shoot_angle_id').value

        GUARD_OPEN = self.get_parameter('guard_open').value
        GUARD_CLOSE = self.get_parameter('guard_close').value
        ARM_OPEN = self.get_parameter('arm_open').value
        ARM_CLOSE = self.get_parameter('arm_close').value
        SHOOT_PUSH_INTAKE_GATE_READY = self.get_parameter('shoot_push_intake_gate_ready').value
        SHOOT_PUSH_INTAKE_SHOOT_READY = self.get_parameter('shoot_push_intake_shoot_ready').value
        BALL_INTAKE_DOWN_ROLLER_SPEED = self.get_parameter('ball_intake_down_roller_speed').value
        BALL_INTAKE_UP_ROLLER_SPEED = self.get_parameter('ball_intake_up_roller_speed').value
        SHOOT_ANGLE_MIN = self.get_parameter('shoot_angle_min').value
        WAIT_TIME_GUARD = self.get_parameter('wait_time_guard').value
        WAIT_TIME_ARM = self.get_parameter('wait_time_arm').value
        WAIT_TIME_SHOOT_DIR = self.get_parameter('wait_time_shoot_dir').value
        WAIT_TIME_PUSH = self.get_parameter('wait_time_push').value
        WAIT_TIME_INTAKE = self.get_parameter('wait_time_intake').value
        

        #左右両方で共通して実行する処理を書く
        #フィードバック無しのため、常に支柱を下げて待機する
        self.get_logger().info("支柱を下げます")
        self.publish_dyna_pos(SHOOT_DIRECTION_ID, SHOOT_ANGLE_MIN)
        await asyncio.sleep(WAIT_TIME_SHOOT_DIR)

        # current_state は 2: LEFT_CARRY, 3: RIGHT_CARRY
        #left
        if current_state == 2:

            #城門
            if execute_mode == 1:
                self.get_logger().info("左脇にあるボールを内側に取り込みます")

                #押し出し機構を上げる
                MINI_SHOOT_CAN_ID = self.get_parameter('mini_shoot_can_id').value
                set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_INTAKE_GATE_READY, CAN_BUS)
                await asyncio.sleep(WAIT_TIME_PUSH)

                #右側のガードを下げる
                self.publish_dyna_pos(RIGHT_GUARD_ID, GUARD_OPEN)

                #左側のガードを上げる
                self.publish_dyna_pos(LEFT_GUARD_ID, GUARD_CLOSE)
                await asyncio.sleep(WAIT_TIME_GUARD)

                #下ローラーを右側へ回転
                set_goal_pwm(DOWN_ROLLER_ID,BALL_INTAKE_DOWN_ROLLER_SPEED,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(UP_LEFT_ROLLER_ID,BALL_INTAKE_UP_ROLLER_SPEED,CAN_BUS)

                #左アームを下ろす
                self.publish_dyna_pos(LEFT_ARM_ID, ARM_CLOSE)
                await asyncio.sleep(WAIT_TIME_ARM)

                #ボールが完全に内側に入るのを待つ
                await asyncio.sleep(WAIT_TIME_INTAKE)
                
                #左ガードを下げる
                self.publish_dyna_pos(LEFT_GUARD_ID, GUARD_OPEN)
                await asyncio.sleep(WAIT_TIME_GUARD)

                #ローラーを止める
                set_goal_pwm(DOWN_ROLLER_ID,0,CAN_BUS)
                set_goal_pwm(UP_LEFT_ROLLER_ID,0,CAN_BUS)

            #射出
            elif execute_mode == 2:
                self.get_logger().info("左脇にあるボールを内側に取り込みます")

                #押し出し機構を下げる
                MINI_SHOOT_CAN_ID = self.get_parameter('mini_shoot_can_id').value
                set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_INTAKE_SHOOT_READY, CAN_BUS)
                await asyncio.sleep(WAIT_TIME_PUSH)

                #右側のガードを下げる
                self.publish_dyna_pos(RIGHT_GUARD_ID, GUARD_OPEN)
                
                #左側のガードを上げる
                self.publish_dyna_pos(LEFT_GUARD_ID, GUARD_CLOSE)
                await asyncio.sleep(WAIT_TIME_GUARD)

                #下ローラーを右側へ回転
                set_goal_pwm(DOWN_ROLLER_ID,BALL_INTAKE_DOWN_ROLLER_SPEED,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(UP_LEFT_ROLLER_ID,BALL_INTAKE_UP_ROLLER_SPEED,CAN_BUS)

                #左アームを下ろす
                self.publish_dyna_pos(LEFT_ARM_ID, ARM_CLOSE)
                await asyncio.sleep(WAIT_TIME_ARM)

                #ボールが完全に内側に入るのを待つ
                await asyncio.sleep(WAIT_TIME_INTAKE)

                #左ガードを下げる
                self.publish_dyna_pos(LEFT_GUARD_ID, GUARD_OPEN)
                await asyncio.sleep(WAIT_TIME_GUARD)

                #ローラーを止める
                set_goal_pwm(DOWN_ROLLER_ID,0,CAN_BUS)
                set_goal_pwm(UP_LEFT_ROLLER_ID,0,CAN_BUS)

        #right
        elif current_state == 3:

            #城門
            if execute_mode == 1:
                self.get_logger().info("右脇にあるボールを内側に取り込みます")

                #押し出し機構を上げる
                MINI_SHOOT_CAN_ID = self.get_parameter('mini_shoot_can_id').value
                set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_INTAKE_GATE_READY, CAN_BUS)
                await asyncio.sleep(WAIT_TIME_PUSH)

                #左側のガードを下げる
                self.publish_dyna_pos(LEFT_GUARD_ID, GUARD_OPEN)

                #右側のガードを上げる
                self.publish_dyna_pos(RIGHT_GUARD_ID, GUARD_CLOSE)
                await asyncio.sleep(WAIT_TIME_GUARD)

                #下ローラーを左側へ回転
                set_goal_pwm(DOWN_ROLLER_ID,BALL_INTAKE_DOWN_ROLLER_SPEED,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(UP_RIGHT_ROLLER_ID,BALL_INTAKE_UP_ROLLER_SPEED,CAN_BUS)

                #右アームを下ろす
                self.publish_dyna_pos(RIGHT_ARM_ID, ARM_CLOSE)
                await asyncio.sleep(WAIT_TIME_ARM)

                #ボールが完全に内側に入るのを待つ
                await asyncio.sleep(WAIT_TIME_INTAKE)

                #右ガードを下げる
                self.publish_dyna_pos(RIGHT_GUARD_ID, GUARD_OPEN)
                await asyncio.sleep(WAIT_TIME_GUARD)

                #ローラーを止める
                set_goal_pwm(DOWN_ROLLER_ID,0)
                set_goal_pwm(UP_RIGHT_ROLLER_ID,0)

            #射出
            elif execute_mode == 2:
                self.get_logger().info("右脇にあるボールを内側に取り込みます")

                #押し出し機構を下げる
                MINI_SHOOT_CAN_ID = self.get_parameter('mini_shoot_can_id').value
                set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_INTAKE_SHOOT_READY, CAN_BUS)
                await asyncio.sleep(WAIT_TIME_PUSH)

                #左側のガードを下げる
                self.publish_dyna_pos(LEFT_GUARD_ID, GUARD_OPEN)

                #右側のガードを上げる
                self.publish_dyna_pos(RIGHT_GUARD_ID, GUARD_CLOSE)
                await asyncio.sleep(WAIT_TIME_GUARD)

                #下ローラーを左側へ回転
                set_goal_pwm(DOWN_ROLLER_ID,BALL_INTAKE_DOWN_ROLLER_SPEED,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(UP_RIGHT_ROLLER_ID,BALL_INTAKE_UP_ROLLER_SPEED,CAN_BUS)

                #右アームを下ろす
                self.publish_dyna_pos(RIGHT_ARM_ID, ARM_CLOSE)
                await asyncio.sleep(WAIT_TIME_ARM)

                #ボールが完全に内側に入るのを待つ
                await asyncio.sleep(WAIT_TIME_INTAKE)

                #右ガードを下げる
                self.publish_dyna_pos(RIGHT_GUARD_ID, GUARD_OPEN)
                await asyncio.sleep(WAIT_TIME_GUARD)

                #ローラーを止める
                set_goal_pwm(DOWN_ROLLER_ID,0)
                set_goal_pwm(UP_RIGHT_ROLLER_ID,0)

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

            success = await self.get_ball(req.current_state, req.execute_mode)

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