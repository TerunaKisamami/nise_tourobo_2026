import rclpy
from rclpy.node import Node
from rclpy.action import ActionServer, GoalResponse
import asyncio
import time

from tourobo_2026_mechanisms.constants import *
from tourobo_2026_interfaces.action import BallVomitPlate
from dyna_interfaces.msg import DynaTarget
from ah_python_lib.ah_python_can import *
import can

CAN_BUS = can.interface.Bus(
    bustype="socketcan", channel="can0", asynchronous=True, bitrate=1000000
)


class BallVomitPlateNode(Node):
    def __init__(self):
        super().__init__("ball_vomit_plate_node")
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()

        self._action_server = ActionServer(
            self,
            BallVomitPlate,
            "ball_vomit_plate",
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        self.dyna_extpos_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_extpos", 10
        )

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

    # 実際の動作部分
    async def execute_vomit_plate_action(self, carry, push_state, shoot_angle_state):

        if carry == BALL_CARRY.LEFT.value:
            self.get_logger().info("左脇抱えなので左側にボールを吐き出します")

            # 左アームを開く
            self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_OPEN)
            time.sleep(WAIT_TIME_ARM)

            # 左上のローラーを外向きに回す
            set_goal_pwm(LEFT_ROLLER_CAN_ID, BALL_VOMIT_UP_ROLLER_SPEED, CAN_BUS)

            # 下のローラーを左向きに回す
            set_goal_pwm(DOWN_ROLLER_CAN_ID, -BALL_VOMIT_DOWN_ROLLER_SPEED, CAN_BUS)

        elif carry == BALL_CARRY.RIGHT.value:
            self.get_logger().info("右脇抱えなので右側にボールを吐き出します")

            # 右アームを開く
            self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_OPEN)
            time.sleep(WAIT_TIME_ARM)

            # 右上のローラーを外向きに回す
            set_goal_pwm(RIGHT_ROLLER_CAN_ID, -BALL_VOMIT_UP_ROLLER_SPEED, CAN_BUS)
            # 下のローラーを右向きに回す
            set_goal_pwm(DOWN_ROLLER_CAN_ID, BALL_VOMIT_DOWN_ROLLER_SPEED, CAN_BUS)

        # 左右共通して行う後処理
        time.sleep(WAIT_TIME_VOMIT)

        # ローラーを止める
        set_goal_pwm(LEFT_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)

        return True

    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            req = goal_handle.request
            res = BallVomitPlate.Result()
            # 初期状態をそのまま引き継ぐ
            res.next_state = req.current_state
            res.next_carry = req.carry
            res.next_push_state = req.push_state
            res.next_shoot_angle_state = req.shoot_angle_state
            res.next_is_left_arm_open = req.is_left_arm_open
            res.next_is_right_arm_open = req.is_right_arm_open

            success = await self.execute_vomit_plate_action(
                req.carry, req.push_state, req.shoot_angle_state
            )

            if success:
                # 実行後は抱えているアームに対してステートを変化させ、抱えを解除する
                if req.carry == BALL_CARRY.LEFT.value:
                    res.next_is_left_arm_open = True  # 仮: アームを開いた状態にする
                elif req.carry == BALL_CARRY.RIGHT.value:
                    res.next_is_right_arm_open = True

                res.next_carry = BALL_CARRY.NOT.value

                # もしSINGLE_CARRY(脇に1つだけ抱えている状態)だったなら、NOT_CARRYに戻す
                if req.current_state == Mechanism_State.SINGLE_CARRY.value:
                    res.next_state = Mechanism_State.NOT_CARRY.value

                res.success = True
                goal_handle.succeed()
            else:
                goal_handle.abort()

            return res
        finally:
            self.is_executing = False


def main(args=None):
    rclpy.init(args=args)
    node = BallVomitPlateNode()
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    try:
        executor.spin()
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()
