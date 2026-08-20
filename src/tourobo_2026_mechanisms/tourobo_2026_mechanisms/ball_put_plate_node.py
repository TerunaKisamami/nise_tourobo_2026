"""
ボールを関所に置く処理
ガード->リンクで閉まるやつ
ゲート->ローラーで入れるやつ(アーム)
"""
from rclpy.action import ActionServer, GoalResponse
import rclpy
from rclpy.node import Node
from .mechanism_base_node import MechanismBaseNode
from std_msgs.msg import String
import asyncio
import os
import sys
import asyncio  

from ah_python_lib.ah_python_can import *
from tourobo_2026_interfaces.action import BallPutPlate
from dyna_interfaces.msg import DynaTarget

CAN_BUS = can.interface.Bus(bustype="socketcan",
                            channel="can0",
                            asynchronous=True,
                            bitrate=1000000)


class BallPutPlateNode(MechanismBaseNode):

    def __init__(self):
        super().__init__('ball_put_plate_node')
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()
        

        
        self._action_server = ActionServer(
            self,
            BallPutPlate,
            'ball_put_plate',
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )


        MINI_SHOOT_CAN_ID = self.get_p('mini_shoot_can_id')
        set_enc_pos_mode(MINI_SHOOT_CAN_ID, CAN_BUS)





    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn('現在別の処理を実行中です。新しい指令を拒否します。')
            return GoalResponse.REJECT
        self.get_logger().info('新しい指令を受け付けました。')
        return GoalResponse.ACCEPT

    #実際の動作部分
    async def put_ball_in_plate(self, current_state):
        LEFT_ARM_ID = self.get_p('arm_left_id')
        RIGHT_ARM_ID = self.get_p('arm_right_id')
        LEFT_GUARD_ID = self.get_p('guard_left_id')
        RIGHT_GUARD_ID = self.get_p('guard_right_id')


        DOWN_ROLLER_ID = self.get_p('down_roller_can_id')
        RIGHT_ROLLER_ID = self.get_p('right_roller_can_id')
        LEFT_ROLLER_ID = self.get_p('left_roller_can_id')

        SHOOT_ANGLE_ID = self.get_p('shoot_angle_id')
        SHOOT_ANGLE_MAX = self.get_p('shoot_angle_max')

        MINI_SHOOT_CAN_ID = self.get_p('mini_shoot_can_id')
        SHOOT_PUSH_MAX = self.get_p('shoot_push_max')

        WAIT_TIME_GUARD = self.get_p('wait_time_guard')

        LEFT_ARM_OPEN = self.get_p('arm_left_open')
        RIGHT_ARM_OPEN = self.get_p('arm_right_open')
        LEFT_ARM_CLOSE = self.get_p('arm_left_close')
        RIGHT_ARM_CLOSE = self.get_p('arm_right_close')
        LEFT_ARM_GET_HALF = self.get_p('arm_left_get_half')
        RIGHT_ARM_GET_HALF = self.get_p('arm_right_get_half')
        LEFT_GUARD_OPEN = self.get_p('guard_left_open')
        RIGHT_GUARD_OPEN = self.get_p('guard_right_open')
        LEFT_GUARD_CLOSE = self.get_p('guard_left_close')
        RIGHT_GUARD_CLOSE = self.get_p('guard_right_close')
        WAIT_TIME_ARM = self.get_p('wait_time_arm')
        WAIT_TIME_PUT_PLATE = self.get_p('wait_time_put_plate')
        WAIT_TIME_SHOOT_DIR = self.get_p('wait_time_shoot_dir')
        WAIT_TIME_PUSH = self.get_p('wait_time_push')

        BALL_PUT_PLATE_DOWN_ROLLER_SPEED = self.get_p('ball_put_plate_down_roller_speed')
        BALL_PUT_PLATE_UP_ROLLER_SPEED = self.get_p('ball_put_plate_up_roller_speed')
        

        #左右共通して行う動作
        #射出機構を上げる
        self.publish_dyna_pos(SHOOT_ANGLE_ID, SHOOT_ANGLE_MAX)
        await asyncio.sleep(WAIT_TIME_SHOOT_DIR)

        #押し出し機構を上に上げる
        set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MAX, CAN_BUS)
        await asyncio.sleep(WAIT_TIME_PUSH)


        # current_state: 2=LEFT_CARRY, 3=RIGHT_CARRY
        if current_state == 3:
            self.get_logger().info("右脇で保持している状態から、左側へボールを関所に置きます")
            #右側だけ半開き(右脇で保持している状態)なら左側から発射

            # 左側のアームを上げる
            self.publish_dyna_pos(LEFT_ARM_ID, LEFT_ARM_OPEN)
            await asyncio.sleep(WAIT_TIME_ARM)

            # 左側のガードを上げる
            self.publish_dyna_pos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
            await asyncio.sleep(WAIT_TIME_GUARD)

            # 左側のローラーを回す
            set_goal_pwm(LEFT_ROLLER_ID,BALL_PUT_PLATE_UP_ROLLER_SPEED,CAN_BUS)

            # 右側のガードを上げる
            self.publish_dyna_pos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
            await asyncio.sleep(WAIT_TIME_GUARD)

            # 右側のローラーを回す
            set_goal_pwm(RIGHT_ROLLER_ID,BALL_PUT_PLATE_UP_ROLLER_SPEED,CAN_BUS)
            # 下のローラーを左向きに回す
            set_goal_pwm(DOWN_ROLLER_ID,BALL_PUT_PLATE_DOWN_ROLLER_SPEED,CAN_BUS)
       
            # 右側のアームを下げる
            self.publish_dyna_pos(RIGHT_ARM_ID, RIGHT_ARM_CLOSE)
            await asyncio.sleep(WAIT_TIME_ARM)

            #ボールが移動して関所に置かれるのを待つ
            await asyncio.sleep(WAIT_TIME_PUT_PLATE)

            #ろーらーをとめる
            set_goal_pwm(LEFT_ROLLER_ID,0,CAN_BUS)
            set_goal_pwm(RIGHT_ROLLER_ID,0,CAN_BUS)
            set_goal_pwm(DOWN_ROLLER_ID,0,CAN_BUS)

        elif current_state == 2:
            self.get_logger().info("左脇で保持している状態から、右側へボールを関所に置きます")
            #左側だけ半開き(左脇で保持している状態)なら右側から発射

            # 右側のアームを上げる
            self.publish_dyna_pos(RIGHT_ARM_ID, RIGHT_ARM_OPEN)
            await asyncio.sleep(WAIT_TIME_ARM)

            # 右側のガードを上げる
            self.publish_dyna_pos(RIGHT_GUARD_ID, RIGHT_GUARD_CLOSE)
            await asyncio.sleep(WAIT_TIME_GUARD)
            # 右側のローラーを回す
            set_goal_pwm(RIGHT_ROLLER_ID,BALL_PUT_PLATE_UP_ROLLER_SPEED,CAN_BUS)

            # 左側のガードを上げる
            self.publish_dyna_pos(LEFT_GUARD_ID, LEFT_GUARD_CLOSE)
            await asyncio.sleep(WAIT_TIME_GUARD)

            # 左側のローラーを回す
            set_goal_pwm(LEFT_ROLLER_ID,BALL_PUT_PLATE_UP_ROLLER_SPEED,CAN_BUS)

            # 下のローラーを右向きに回す
            set_goal_pwm(DOWN_ROLLER_ID,BALL_PUT_PLATE_DOWN_ROLLER_SPEED,CAN_BUS)
            
            # 左側のアームを下ろす
            self.publish_dyna_pos(LEFT_ARM_ID, LEFT_ARM_CLOSE)
            await asyncio.sleep(WAIT_TIME_ARM)

            #ボールが移動して関所に置かれるのを待つ
            await asyncio.sleep(WAIT_TIME_PUT_PLATE)

            #ローラーを止める
            set_goal_pwm(LEFT_ROLLER_ID,0,CAN_BUS)
            set_goal_pwm(RIGHT_ROLLER_ID,0,CAN_BUS)
            set_goal_pwm(DOWN_ROLLER_ID,0,CAN_BUS)

        else:
            self.get_logger().error(
                f"エラー: 想定外の current_state ({current_state}) です。")
            return False

        #終了処理
        # 左側の上のローラーを止める
        # 右側の上のローラーを止める
        # 下のローラーを止める
        # ガードを落とす

        self.get_logger().info("関所への配置完了")
        return True

    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallPutPlate.Result()
            success = False

            success = await self.put_ball_in_plate(req.current_state)

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
    node = BallPutPlateNode()
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    executor.spin()
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
