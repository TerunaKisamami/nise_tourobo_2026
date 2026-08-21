"""
わきにかかえたボールを内側へ取り込む
"""

from rclpy.action import ActionServer, GoalResponse
import rclpy
from rclpy.node import Node
from .mechanism_base_node import MechanismBaseNode
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


class BallIntakeNode(MechanismBaseNode):

    def __init__(self):
        super().__init__('ball_intake_node')
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()





        self._action_server = ActionServer(
            self,
            BallIntake,
            'ball_intake',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )


        #dcモーター立ち上げ
        set_pwm_mode(self.p.down_roller_can_id, CAN_BUS)
        set_pwm_mode(self.p.right_roller_can_id, CAN_BUS)
        set_pwm_mode(self.p.left_roller_can_id, CAN_BUS)

        #dcモーター初期化
        set_goal_pwm(self.p.down_roller_can_id, 0, CAN_BUS)
        set_goal_pwm(self.p.right_roller_can_id, 0, CAN_BUS)
        set_goal_pwm(self.p.left_roller_can_id, 0, CAN_BUS)

        #ロボマスモーター立ち上げ
        set_enc_pos_mode(self.p.mini_shoot_can_id, CAN_BUS)

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


    # ここがメインの処理じゃぞ
    async def get_ball(self, current_state, execute_mode):

        #ダイナミクセルのID

        # ローラー用サーボモーターID

        #発射機構のID


        
        

        #左右両方で共通して実行する処理を書く
        #フィードバック無しのため、常に支柱を下げて待機する
        self.get_logger().info("支柱を下げます")
        self.publish_dyna_extpos(self.p.shoot_direction_id, self.p.shoot_angle_min)
        await asyncio.sleep(self.p.wait_time_shoot_dir)

        # current_state は 2: self.p.left_carry, 3: self.p.right_carry
        #left
        if current_state == 2:

            #城門
            if execute_mode == 1:
                self.get_logger().info("左脇にあるボールを内側に取り込みます")

                #押し出し機構を上げる
                set_goal_pos(self.p.mini_shoot_can_id, self.p.shoot_push_intake_gate_ready, CAN_BUS)
                await asyncio.sleep(self.p.wait_time_push)

                #右側のガードを下げる
                self.publish_dyna_extpos(self.p.right_guard_id, self.p.guard_right_open)

                #左側のガードを上げる
                self.publish_dyna_extpos(self.p.left_guard_id, self.p.guard_left_close)
                await asyncio.sleep(self.p.wait_time_guard)

                #下ローラーを右側へ回転
                set_goal_pwm(self.p.down_roller_id,self.p.ball_intake_down_roller_speed,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(self.p.up_left_roller_id,self.p.ball_intake_up_roller_speed,CAN_BUS)

                #左アームを下ろす
                self.publish_dyna_extpos(self.p.left_arm_id, self.p.arm_left_close)
                await asyncio.sleep(self.p.wait_time_arm)

                #ボールが完全に内側に入るのを待つ
                await asyncio.sleep(self.p.wait_time_intake)
                
                #左ガードを下げる
                self.publish_dyna_extpos(self.p.left_guard_id, self.p.guard_left_open)
                await asyncio.sleep(self.p.wait_time_guard)

                #ローラーを止める
                set_goal_pwm(self.p.down_roller_id,0,CAN_BUS)
                set_goal_pwm(self.p.up_left_roller_id,0,CAN_BUS)

            #射出
            elif execute_mode == 2:
                self.get_logger().info("左脇にあるボールを内側に取り込みます")

                #押し出し機構を下げる
                set_goal_pos(self.p.mini_shoot_can_id, self.p.shoot_push_intake_shoot_ready, CAN_BUS)
                await asyncio.sleep(self.p.wait_time_push)

                #右側のガードを下げる
                self.publish_dyna_extpos(self.p.right_guard_id, self.p.guard_right_open)
                
                #左側のガードを上げる
                self.publish_dyna_extpos(self.p.left_guard_id, self.p.guard_left_close)
                await asyncio.sleep(self.p.wait_time_guard)

                #下ローラーを右側へ回転
                set_goal_pwm(self.p.down_roller_id,self.p.ball_intake_down_roller_speed,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(self.p.up_left_roller_id,self.p.ball_intake_up_roller_speed,CAN_BUS)

                #左アームを下ろす
                self.publish_dyna_extpos(self.p.left_arm_id, self.p.arm_left_close)
                await asyncio.sleep(self.p.wait_time_arm)

                #ボールが完全に内側に入るのを待つ
                await asyncio.sleep(self.p.wait_time_intake)

                #左ガードを下げる
                self.publish_dyna_extpos(self.p.left_guard_id, self.p.guard_left_open)
                await asyncio.sleep(self.p.wait_time_guard)

                #ローラーを止める
                set_goal_pwm(self.p.down_roller_id,0,CAN_BUS)
                set_goal_pwm(self.p.up_left_roller_id,0,CAN_BUS)

        #right
        elif current_state == 3:

            #城門
            if execute_mode == 1:
                self.get_logger().info("右脇にあるボールを内側に取り込みます")

                #押し出し機構を上げる
                set_goal_pos(self.p.mini_shoot_can_id, self.p.shoot_push_intake_gate_ready, CAN_BUS)
                await asyncio.sleep(self.p.wait_time_push)

                #左側のガードを下げる
                self.publish_dyna_extpos(self.p.left_guard_id, self.p.guard_left_open)

                #右側のガードを上げる
                self.publish_dyna_extpos(self.p.right_guard_id, self.p.guard_right_close)
                await asyncio.sleep(self.p.wait_time_guard)

                #下ローラーを左側へ回転
                set_goal_pwm(self.p.down_roller_id,self.p.ball_intake_down_roller_speed,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(self.p.up_right_roller_id,self.p.ball_intake_up_roller_speed,CAN_BUS)

                #右アームを下ろす
                self.publish_dyna_extpos(self.p.right_arm_id, self.p.arm_right_close)
                await asyncio.sleep(self.p.wait_time_arm)

                #ボールが完全に内側に入るのを待つ
                await asyncio.sleep(self.p.wait_time_intake)

                #右ガードを下げる
                self.publish_dyna_extpos(self.p.right_guard_id, self.p.guard_right_open)
                await asyncio.sleep(self.p.wait_time_guard)

                #ローラーを止める
                set_goal_pwm(self.p.down_roller_id, 0, CAN_BUS)
                set_goal_pwm(self.p.up_right_roller_id, 0, CAN_BUS)

            #射出
            elif execute_mode == 2:
                self.get_logger().info("右脇にあるボールを内側に取り込みます")

                #押し出し機構を下げる
                set_goal_pos(self.p.mini_shoot_can_id, self.p.shoot_push_intake_shoot_ready, CAN_BUS)
                await asyncio.sleep(self.p.wait_time_push)

                #左側のガードを下げる
                self.publish_dyna_extpos(self.p.left_guard_id, self.p.guard_left_open)

                #右側のガードを上げる
                self.publish_dyna_extpos(self.p.right_guard_id, self.p.guard_right_close)
                await asyncio.sleep(self.p.wait_time_guard)

                #下ローラーを左側へ回転
                set_goal_pwm(self.p.down_roller_id,self.p.ball_intake_down_roller_speed,CAN_BUS)

                #上ローラーを回転
                set_goal_pwm(self.p.up_right_roller_id,self.p.ball_intake_up_roller_speed,CAN_BUS)

                #右アームを下ろす
                self.publish_dyna_extpos(self.p.right_arm_id, self.p.arm_right_close)
                await asyncio.sleep(self.p.wait_time_arm)

                #ボールが完全に内側に入るのを待つ
                await asyncio.sleep(self.p.wait_time_intake)

                #右ガードを下げる
                self.publish_dyna_extpos(self.p.right_guard_id, self.p.guard_right_open)
                await asyncio.sleep(self.p.wait_time_guard)

                #ローラーを止める
                set_goal_pwm(self.p.down_roller_id, 0, CAN_BUS)
                set_goal_pwm(self.p.up_right_roller_id, 0, CAN_BUS)

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
                    res.next_state = 4  # self.p.intake
                #射出
                elif (req.execute_mode == 2):
                    res.next_state = 5  # self.p.intake
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