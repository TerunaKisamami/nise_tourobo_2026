import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from rclpy.callback_groups import ReentrantCallbackGroup
from sensor_msgs.msg import Joy
from action_msgs.msg import GoalStatus
from dataclasses import dataclass

from tourobo_2026_mechanisms.constants import *
from tourobo_2026_interfaces.action import *


@dataclass
class RobotState:
    current: Mechanism_State = Mechanism_State.UNKNOWN
    carry: BALL_CARRY = BALL_CARRY.NOT
    shoot_push: Shoot_Push_State = Shoot_Push_State.MIN
    shoot_angle: Shoot_Angle_State = Shoot_Angle_State.MIN
    ball_carry: BALL_CARRY = BALL_CARRY.NOT
    is_left_arm_open: bool = False
    is_right_arm_open: bool = False


class JoyMechanismClient(Node):
    def __init__(self):
        super().__init__("joy_mechanism_client")
        self.cb_group = ReentrantCallbackGroup()

        self.joy_sub = self.create_subscription(
            Joy, "/joy", self.joy_callback, 10, callback_group=self.cb_group
        )

        # 論理状態の初期化
        self.state = RobotState()

        # クライアント設定
        self.ball_get_client = ActionClient(
            self, BallGet, "ball_get", callback_group=self.cb_group
        )
        self.ball_put_gate_client = ActionClient(
            self, BallPutGate, "ball_put_gate", callback_group=self.cb_group
        )
        self.ball_put_plate_client = ActionClient(
            self, BallPutPlate, "ball_put_plate", callback_group=self.cb_group
        )
        self.ball_shoot_client = ActionClient(
            self, BallShoot, "ball_shoot", callback_group=self.cb_group
        )
        self.ball_arm_operation_client = ActionClient(
            self, BallArmOperation, "ball_arm_operation", callback_group=self.cb_group
        )
        self.ball_intake_client = ActionClient(
            self, BallIntake, "ball_intake", callback_group=self.cb_group
        )
        self.mechanism_reset_client = ActionClient(
            self, MechanismReset, "mechanism_reset", callback_group=self.cb_group
        )

        # ボタンの状態保持用
        self.prev_buttons = []
        self.prev_axes = []

        # 連打防止用のフラグ
        self.is_action_running = False

    def set_goal_states(self, goal_msg):
        goal_msg.current_state = self.state.current.value
        goal_msg.carry = self.state.carry.value
        goal_msg.push_state = self.state.shoot_push.value
        goal_msg.shoot_angle_state = self.state.shoot_angle.value
        goal_msg.is_left_arm_open = self.state.is_left_arm_open
        goal_msg.is_right_arm_open = self.state.is_right_arm_open

    def update_states(self, res):
        if res and hasattr(res, 'success') and res.success:
            if hasattr(res, 'next_state') and res.next_state != Mechanism_State.UNKNOWN.value:
                self.state.current = Mechanism_State(res.next_state)
            if hasattr(res, 'next_carry'):
                self.state.carry = BALL_CARRY(res.next_carry)
            if hasattr(res, 'next_push_state'):
                self.state.shoot_push = Shoot_Push_State(res.next_push_state)
            if hasattr(res, 'next_shoot_angle_state'):
                self.state.shoot_angle = Shoot_Angle_State(res.next_shoot_angle_state)
            if hasattr(res, 'next_is_left_arm_open'):
                self.state.is_left_arm_open = res.next_is_left_arm_open
            if hasattr(res, 'next_is_right_arm_open'):
                self.state.is_right_arm_open = res.next_is_right_arm_open

    # action送信共通関数
    async def send_action_goal(self, client, goal_msg, action_name):
        self.get_logger().info(f"[{action_name}] サーバーを待機中...")
        if not client.wait_for_server(timeout_sec=1.0):
            self.get_logger().error(f"[{action_name}] サーバーが見つかりません")
            return None

        self.get_logger().info(f"[{action_name}] ゴール送信 (state: {self.state})")
        send_goal_future = await client.send_goal_async(goal_msg)

        if not send_goal_future.accepted:
            self.get_logger().error(f"[{action_name}] 命令が拒否されました")
            return None

        self.get_logger().info(f"[{action_name}] 受理されました。結果を待機します...")
        result_handle = await send_goal_future.get_result_async()

        if result_handle.status == GoalStatus.STATUS_SUCCEEDED:
            self.get_logger().info(f"[{action_name}] 正常完了しました")
            return result_handle.result
        else:
            self.get_logger().warn(
                f"[{action_name}] 失敗しました (Status ID: {result_handle.status})"
            )
            return None

    # 各種アクションのゴール生成
    async def send_ball_get_goal(self, execute_mode):
        goal_msg = BallGet.Goal()
        goal_msg.execute_mode = execute_mode
        self.set_goal_states(goal_msg)
        return await self.send_action_goal(self.ball_get_client, goal_msg, "ball_get")

    async def send_ball_intake_goal(self, execute_mode):
        goal_msg = BallIntake.Goal()
        goal_msg.execute_mode = execute_mode
        self.set_goal_states(goal_msg)
        return await self.send_action_goal(
            self.ball_intake_client, goal_msg, "ball_intake"
        )

    async def send_ball_put_gate_goal(self):
        goal_msg = BallPutGate.Goal()
        goal_msg.execute = True
        self.set_goal_states(goal_msg)
        return await self.send_action_goal(
            self.ball_put_gate_client, goal_msg, "ball_put_gate"
        )

    async def send_ball_put_plate_goal(self):
        goal_msg = BallPutPlate.Goal()
        goal_msg.execute = True
        self.set_goal_states(goal_msg)
        return await self.send_action_goal(
            self.ball_put_plate_client, goal_msg, "ball_put_plate"
        )

    async def send_ball_shoot_goal(self):
        goal_msg = BallShoot.Goal()
        goal_msg.execute = True
        self.set_goal_states(goal_msg)
        return await self.send_action_goal(
            self.ball_shoot_client, goal_msg, "ball_shoot"
        )

    async def send_ball_arm_operation_goal(self, target_arm, is_open):
        goal_msg = BallArmOperation.Goal()
        goal_msg.target_arm = target_arm
        goal_msg.is_open = is_open
        self.set_goal_states(goal_msg)
        return await self.send_action_goal(
            self.ball_arm_operation_client, goal_msg, "ball_arm_operation"
        )

    async def send_mechanism_reset_goal(self):
        goal_msg = MechanismReset.Goal()
        goal_msg.execute = True
        return await self.send_action_goal(
            self.mechanism_reset_client, goal_msg, "mechanism_reset"
        )

    # ボタンが押された瞬間だけ取る
    def is_pressed(self, msg, prev_buttons, idx):
        if idx < len(msg.buttons) and idx < len(prev_buttons):
            return msg.buttons[idx] == 1 and prev_buttons[idx] == 0
        return False

    # スティックが一定角度以上傾いたときだけとる
    def is_axis_changed(self, msg, prev_axes, idx, threshold, direction):
        if idx < len(msg.axes) and idx < len(prev_axes):
            if direction > 0:
                return msg.axes[idx] > threshold and prev_axes[idx] <= threshold
            else:
                return msg.axes[idx] < -threshold and prev_axes[idx] >= -threshold
        return False

    # joyスティックが入力されるたびに呼ばれるやつ
    async def joy_callback(self, msg):
        # 初期化だけする
        if not self.prev_buttons:
            self.prev_buttons = msg.buttons
            self.prev_axes = msg.axes
            return

        # 以前の状態をローカルに保存してから上書きする (await後の巻き戻りを防ぐため)
        prev_buttons = self.prev_buttons
        prev_axes = self.prev_axes
        self.prev_buttons = msg.buttons
        self.prev_axes = msg.axes

        # 各ボタン機構
        # OPTIONS: 強制リセット
        # □: ボールを内側に取り込む(射撃用)→もう一度押すとボールを射出
        # △: ボールを内側に取り込む(城門用)→もう一度押すと城門に設置
        # ◯: 今抱えている方向の反対側へボールを関所に配置
        # L1: 左ゲートを開閉
        # R1: 右ゲートを開閉
        # L2: 左から脇に抱える
        # R2: 右から脇に抱える

        # ボタンが押された瞬間に一括でステートをログ表示する
        if not self.is_action_running and any(self.is_pressed(msg, prev_buttons, b) for b in [0, 2, 3, 4, 5, 6, 7, 9]):
            state_str = (
                f"\n  current: {self.state.current.name}"
                f"\n  carry: {self.state.carry.name}"
                f"\n  shoot_push: {self.state.shoot_push.name}"
                f"\n  shoot_angle: {self.state.shoot_angle.name}"
                f"\n  left_arm_open: {self.state.is_left_arm_open}"
                f"\n  right_arm_open: {self.state.is_right_arm_open}"
            )
            self.get_logger().info(f"--- Button Pressed --- state = {state_str}")

        if self.is_pressed(msg, prev_buttons, 9):  # OPTIONSボタンで強制リセット
            self.get_logger().info(
                "OPTIONSボタン: 機構の初期位置へのリセットを開始します"
            )
            self.is_action_running = True
            res = await self.send_mechanism_reset_goal()
            if res and res.success:
                self.get_logger().info("ハードウェアのリセットが完了しました。")
                self.update_states(res)
            else:
                self.get_logger().error(
                    "ハードウェアのリセットに失敗しました！論理状態のみリセットします"
                )
                self.state = RobotState()
            self.is_action_running = False

        elif self.is_action_running:
            # 動作中は何のボタンを押しても受け付けない
            # 何かボタンや軸が操作されたときだけ警告を出す
            if any(self.is_pressed(msg, prev_buttons, b) for b in [2, 3, 4, 5, 6, 7]):
                self.get_logger().warn("現在他の動作中です")

        # × :ボールを城門に入れる
        elif self.is_pressed(msg, prev_buttons, 0):
            self.get_logger().info("×ボタンが入力された")

            # current_stateがLEFT_CARRYならば右の関所へ
            if (
                self.state.current == Mechanism_State.SINGLE_CARRY
                and self.state.carry == BALL_CARRY.LEFT
            ):
                self.is_action_running = True
                res = await self.send_ball_put_plate_goal()

                if res and res.success:
                    self.update_states(res)
                self.is_action_running = False

            # current_stateがRIGHT_CARRYならば左の関所へ
            elif (
                self.state.current == Mechanism_State.SINGLE_CARRY
                and self.state.carry == BALL_CARRY.RIGHT
            ):
                self.is_action_running = True
                res = await self.send_ball_put_plate_goal()

                if res and res.success:
                    self.update_states(res)
                self.is_action_running = False

        # △: ボールを城門に入れる
        elif self.is_pressed(msg, prev_buttons, 2):
            self.get_logger().info("△ボタンが入力された")

            # current_stateがLEFT_CARRYかRIGHT_CARRYならば城門用内側取り込み
            if self.state.current == Mechanism_State.SINGLE_CARRY:
                self.is_action_running = True
                res = await self.send_ball_intake_goal(1)

                if res and res.success:
                    self.update_states(res)
                self.is_action_running = False

            # current_stateがINTAKE_GATEならば城門に置く
            elif self.state.current == Mechanism_State.INTAKE_GATE:
                self.is_action_running = True
                res = await self.send_ball_put_gate_goal()

                if res and res.success:
                    self.update_states(res)
                self.is_action_running = False

        # □: 射出用取り込み + ボールを射出
        elif self.is_pressed(msg, prev_buttons, 3):
            self.get_logger().info("□ボタンが入力された")

            # current_stateがLEFT_CARRYかRIGHT_CARRYならば内側取り込み
            if self.state.current == Mechanism_State.SINGLE_CARRY:
                self.is_action_running = True
                res = await self.send_ball_intake_goal(2)

                if res and res.success:
                    self.update_states(res)
                self.is_action_running = False

            # current_stateがなら射出
            elif self.state.current == Mechanism_State.INTAKE_SHOOT:
                self.is_action_running = True
                res = await self.send_ball_shoot_goal()

                if res and res.success:
                    self.update_states(res)
                self.is_action_running = False

        # L1: 左ゲートを開閉
        elif self.is_pressed(msg, prev_buttons, 4):
            self.get_logger().info("L1が入力された")
            self.is_action_running = True
            target_is_open = not self.state.is_left_arm_open
            res = await self.send_ball_arm_operation_goal(1, target_is_open)

            if res and res.success:
                self.update_states(res)
            self.is_action_running = False

        # R1: 右ゲートを開閉
        elif self.is_pressed(msg, prev_buttons, 5):
            self.get_logger().info("R1が入力された")
            self.is_action_running = True
            target_is_open = not self.state.is_right_arm_open
            res = await self.send_ball_arm_operation_goal(2, target_is_open)
            if res and res.success:
                self.update_states(res)
            self.is_action_running = False

        # L2: 左から脇に抱える
        elif self.is_pressed(msg, prev_buttons, 6):
            if not self.state.is_left_arm_open:
                self.get_logger().warn(
                    "左アームが開いていないため、BallGet(左)は実行できません"
                )
                return

            if self.state.current in [
                Mechanism_State.NOT_CARRY,
                Mechanism_State.INTAKE_GATE,
                Mechanism_State.INTAKE_SHOOT,
                Mechanism_State.UNKNOWN,
            ]:
                self.get_logger().info("L2が入力された")
                self.is_action_running = True
                res = await self.send_ball_get_goal(1)
                if res and res.success:
                    self.update_states(res)
                self.is_action_running = False

        # R2: 右から脇に抱える
        elif self.is_pressed(msg, prev_buttons, 7):
            if not self.state.is_right_arm_open:
                self.get_logger().warn(
                    "右アームが開いていないため、BallGet(右)は実行できません"
                )
                return

            if self.state.current in [
                Mechanism_State.NOT_CARRY,
                Mechanism_State.INTAKE_GATE,
                Mechanism_State.INTAKE_SHOOT,
                Mechanism_State.UNKNOWN,
            ]:
                self.get_logger().info("R2が入力された")
                self.is_action_running = True
                res = await self.send_ball_get_goal(2)
                if res and res.success:
                    self.update_states(res)
                self.is_action_running = False


def main(args=None):
    rclpy.init(args=args)
    node = JoyMechanismClient()
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
