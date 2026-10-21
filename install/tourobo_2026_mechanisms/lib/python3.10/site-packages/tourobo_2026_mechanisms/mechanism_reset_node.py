import rclpy
from rclpy.node import Node
from rclpy.action import ActionServer, GoalResponse
import asyncio
import time

from tourobo_2026_mechanisms.constants import *
from tourobo_2026_interfaces.action import MechanismReset
from dyna_interfaces.msg import DynaTarget
from ah_python_lib.ah_python_can import *
import can

CAN_BUS = can.interface.Bus(
    bustype="socketcan", channel="can0", asynchronous=True, bitrate=1000000
)

class MechanismResetNode(Node):
    def __init__(self):
        super().__init__("mechanism_reset_node")
        self.is_executing = False
        self.cb_group = rclpy.callback_groups.ReentrantCallbackGroup()

        self._action_server = ActionServer(
            self,
            MechanismReset,
            "mechanism_reset",
            self.execute_callback,
            goal_callback=self.goal_callback,
            callback_group=self.cb_group,
        )

        set_enc_pos_mode(MINI_SHOOT_CAN_ID, CAN_BUS)

        self.dyna_extpos_publisher = self.create_publisher(
            DynaTarget, "/dyna_target_extpos", 10
        )

    def goal_callback(self, goal_request):
        if self.is_executing:
            self.get_logger().warn("現在別の処理を実行中です。新しい指令を拒否します。")
            return GoalResponse.REJECT
        self.get_logger().info("強制リセット指令を受け付けました。")
        return GoalResponse.ACCEPT

    def publish_dyna_extpos(self, id, target):
        msg = DynaTarget()
        msg.id = id
        msg.target = target
        self.dyna_extpos_publisher.publish(msg)

    def reset_mechanisms_sync(self):
        self.get_logger().info("終了処理: 機構を初期位置に戻します (shoot_angleは除く)")

        # 左右アームを閉じる
        self.publish_dyna_extpos(LEFT_ARM_ID, LEFT_ARM_CLOSE)
        self.publish_dyna_extpos(RIGHT_ARM_ID, RIGHT_ARM_CLOSE)
        
        # 左右ガードを開ける
        self.publish_dyna_extpos(LEFT_GUARD_ID, LEFT_GUARD_OPEN)
        self.publish_dyna_extpos(RIGHT_GUARD_ID, RIGHT_GUARD_OPEN)
        
        # モーターが動くのを待つ
        time.sleep(WAIT_TIME_GUARD)

        # ローラーを全て停止する
        set_goal_pwm(LEFT_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(RIGHT_ROLLER_CAN_ID, 0, CAN_BUS)
        set_goal_pwm(DOWN_ROLLER_CAN_ID, 0, CAN_BUS)

        # 押し出し機構を初期位置に戻す
        set_goal_pos(MINI_SHOOT_CAN_ID, SHOOT_PUSH_MIN, CAN_BUS)
        time.sleep(WAIT_TIME_PUSH)

    async def reset_mechanisms(self):
        self.reset_mechanisms_sync()
        return True

    async def execute_callback(self, goal_handle):
        self.is_executing = True
        try:
            res = MechanismReset.Result()
            success = await self.reset_mechanisms()

            if success:
                res.success = True
                res.next_state = Mechanism_State.NOT_CARRY.value
                res.next_carry = BALL_CARRY.NOT.value
                res.next_push_state = Shoot_Push_State.MIN.value
                res.next_is_left_arm_open = False
                res.next_is_right_arm_open = False
                goal_handle.succeed()
            else:
                goal_handle.abort()

            return res
        finally:
            self.is_executing = False

def main(args=None):
    rclpy.init(args=args)
    node = MechanismResetNode()
    executor = rclpy.executors.MultiThreadedExecutor()
    executor.add_node(node)
    try:
        executor.spin()
    except KeyboardInterrupt:
        node.get_logger().info("Ctrl+Cが押されました。終了時のリセット処理を実行します...")
        node.reset_mechanisms_sync()
    finally:
        node.destroy_node()
        rclpy.shutdown()

if __name__ == "__main__":
    main()
