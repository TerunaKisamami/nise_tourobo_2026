import can
from rclpy.action import ActionServer, GoalResponse
import rclpy
from rclpy.node import Node
from tourobo_2026_mechanisms.constants import *
from tourobo_2026_mechanisms.joy_mechanism_client import Shoot_Push_State
from std_msgs.msg import String
import asyncio
import os
import sys
import asyncio  
import time

from ah_python_lib.ah_python_can import *
from tourobo_2026_interfaces.action import BallPutPlate
from dyna_interfaces.msg import DynaTarget

CAN_BUS = can.interface.Bus(bustype="socketcan",
                            channel="can0",
                            asynchronous=True,
                            bitrate=1000000)

class BallPutPlateNode(Node):

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

        set_enc_pos_mode(MINI_SHOOT_CAN_ID, CAN_BUS)

        self.dyna_extpos_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_extpos", 10)
        self.dyna_vel_publisher = self.create_publisher(DynaTarget,
                                                        "/dyna_target_vel", 10)
        self.dyna_pos_publisher = self.create_publisher(DynaTarget,
                                                        "/dyna_target_pos", 10)

    def publish_dyna_pos(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_pos_publisher.publish(msg)

    #相対角度
    def publish_dyna_extpos(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_extpos_publisher.publish(msg)

    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn('現在別の処理を実行中です。新しい指令を拒否します。')
            return GoalResponse.REJECT
        self.get_logger().info('新しい指令を受け付けました。')
        return GoalResponse.ACCEPT

    #実際の動作部分
    async def put_ball_in_plate(self, current_state, push_state):


        #左右共通して行う動作
        #押し出し機構を上に上げる
        if push_state != Shoot_Push_State.MAX.value:
            set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MAX, CAN_BUS)
        time.sleep(WAIT_TIME_PUSH)

        #射出機構を上げる
        self.publish_dyna_extpos(SHOOT_ANGLE_ID, SHOOT_ANGLE_MIN)
        time.sleep(WAIT_TIME_SHOOT_ANGLE)

        # current_state: 2=LEFT_CARRY, 3=RIGHT_CARRY
        if current_state == 3:
            self.get_logger().info("右脇で保持している状態から、左側へボールを関所に置きます")
            #右側だけ半開き(右脇で保持している状態)なら左側から発射

            # 左側のアームを上げる
            self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_OPEN)
            time.sleep(WAIT_TIME_ARM)

            # 左側のガードを上げる
            self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_OPEN)
            time.sleep(WAIT_TIME_GUARD)

            # 左側のローラーを回す
            set_goal_pwm(LEFT_ROLLER_CAN_ID,BALL_PUT_PLATE_UP_ROLLER_SPEED,CAN_BUS)

            # 右側のガードを上げる
            self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_OPEN)
            time.sleep(WAIT_TIME_GUARD)

            # 右側のローラーを回す
            set_goal_pwm(RIGHT_ROLLER_CAN_ID,BALL_PUT_PLATE_UP_ROLLER_SPEED,CAN_BUS)
            # 下のローラーを左向きに回す
            set_goal_pwm(DOWN_ROLLER_CAN_ID,-BALL_PUT_PLATE_DOWN_ROLLER_SPEED,CAN_BUS)
       
            # 右側のアームを下げる
            self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_CLOSE)
            time.sleep(WAIT_TIME_ARM)

            #ボールが移動して関所に置かれるのを待つ
            time.sleep(WAIT_TIME_PUT_PLATE)

            #ろーらーをとめ
            set_goal_pwm(LEFT_ROLLER_CAN_ID,0,CAN_BUS)
            set_goal_pwm(RIGHT_ROLLER_CAN_ID,0,CAN_BUS)
            set_goal_pwm(DOWN_ROLLER_CAN_ID,0,CAN_BUS)

        elif current_state == 2:
            self.get_logger().info("左脇で保持している状態から、右側へボールを関所に置きます")
            #左側だけ半開き(左脇で保持している状態)なら右側から発射

            # 右側のアームを上げる
            self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_OPEN)
            time.sleep(WAIT_TIME_ARM)

            # 右側のガードを上げる
            self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_OPEN)
            time.sleep(WAIT_TIME_GUARD)
            # 右側のローラーを回す
            set_goal_pwm(RIGHT_ROLLER_CAN_ID,BALL_PUT_PLATE_UP_ROLLER_SPEED,CAN_BUS)

            # 左側のガードを上げる
            self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_OPEN)
            time.sleep(WAIT_TIME_GUARD)

            # 左側のローラーを回す
            set_goal_pwm(LEFT_ROLLER_CAN_ID,BALL_PUT_PLATE_UP_ROLLER_SPEED,CAN_BUS)

            # 下のローラーを右向きに回す
            set_goal_pwm(DOWN_ROLLER_CAN_ID,BALL_PUT_PLATE_DOWN_ROLLER_SPEED,CAN_BUS)
            
            # 左側のアームを下ろす
            self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_CLOSE)
            time.sleep(WAIT_TIME_ARM)

            #ボールが移動して関所に置かれるのを待つ
            time.sleep(WAIT_TIME_PUT_PLATE)

            #ローラーを止める
            set_goal_pwm(LEFT_ROLLER_CAN_ID,0,CAN_BUS)
            set_goal_pwm(RIGHT_ROLLER_CAN_ID,0,CAN_BUS)
            set_goal_pwm(DOWN_ROLLER_CAN_ID,0,CAN_BUS)

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

            success = await self.put_ball_in_plate(req.current_state, req.push_state)

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
