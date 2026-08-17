from sympy import false
import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from rclpy.callback_groups import ReentrantCallbackGroup
from sensor_msgs.msg import Joy
from action_msgs.msg import GoalStatus
from enum import Enum

from tourobo_2026_interfaces.action import BallGet, BallPutGate, BallPutPlate, BallShoot, BallShootAim, BallGateOperation, BallIntake

class Mechanism_State(Enum):
    UNKNOWN = 0
    # 最初
    # できること
        # 左または右のどちらかのゲートを開閉
        # 左または右からボールを脇にかかえる動作
            # LEFT_CARRYかRIGHT_CARRYへ
    NOT_CARRY = 1

    #ボールを脇に保持
    #できること
        #ボールを反対側から排出
            #NOT_CARRYへ
        #ボールを内側に取り込む
            #INTAKEへ
        #ゲートの開閉
            #保持している方向のゲートが開いたら NOT_CARRYへ
    LEFT_CARRY = 2
    RIGHT_CARRY = 3

    #ボールを内側に保持
    #できること
        #ボールを発射
            #NOT_CARRYへ
        #ボールを城門に置く
            #NOT_CARRYへ
        #ゲートの開閉
        #射出機構を上下する
    INTAKE = 4


class JoyMechanismClient(Node):

    def __init__(self):
        super().__init__('joy_mechanism_client')
        self.cb_group = ReentrantCallbackGroup()

        self.joy_sub = self.create_subscription(
            Joy,
            '/joy',
            self.joy_callback,
            10,
            callback_group=self.cb_group
        )

        # 論理状態の初期化
        self.current_state = Mechanism_State.UNKNOWN
        self.is_left_gate_open = False
        self.is_right_gate_open = False

        # クライアント設定
        self.ball_get_client = ActionClient(self, BallGet, 'ball_get', callback_group=self.cb_group)
        self.ball_put_gate_client = ActionClient(self, BallPutGate, 'ball_put_gate', callback_group=self.cb_group)
        self.ball_put_plate_client = ActionClient(self, BallPutPlate, 'ball_put_plate', callback_group=self.cb_group)
        self.ball_shoot_client = ActionClient(self, BallShoot, 'ball_shoot', callback_group=self.cb_group)
        self.ball_shoot_aim_client = ActionClient(self, BallShootAim, 'ball_shoot_aim', callback_group=self.cb_group)
        self.ball_gate_operation_client = ActionClient(self, BallGateOperation, 'ball_gate_operation', callback_group=self.cb_group)
        self.ball_intake_client = ActionClient(self, BallIntake, 'ball_intake', callback_group=self.cb_group)

        # ボタンの状態保持用
        self.prev_buttons = []
        self.prev_axes = []
        
        # 連打防止用のフラグ
        self.is_action_running = False

    # action送信共通関数
    async def send_action_goal(self, client, goal_msg, action_name):
        self.get_logger().info(f"[{action_name}] サーバーを待機中...")
        if not client.wait_for_server(timeout_sec=1.0):
            self.get_logger().error(f"[{action_name}] サーバーが見つかりません")
            return None

        self.get_logger().info(f"[{action_name}] ゴール送信 (current_state: {self.current_state.name})")
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
            self.get_logger().warn(f"[{action_name}] 失敗しました (Status ID: {result_handle.status})")
            return None

    # 各種アクションのゴール生成
    async def send_ball_get_goal(self, execute_mode):
        goal_msg = BallGet.Goal()
        goal_msg.execute_mode = execute_mode
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_get_client, goal_msg, "ball_get")

    async def send_ball_put_gate_goal(self):
        goal_msg = BallPutGate.Goal()
        goal_msg.execute = True
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_put_gate_client, goal_msg, "ball_put_gate")

    async def send_ball_put_plate_goal(self):
        goal_msg = BallPutPlate.Goal()
        goal_msg.execute = True
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_put_plate_client, goal_msg, "ball_put_plate")

    async def send_ball_shoot_goal(self):
        goal_msg = BallShoot.Goal()
        goal_msg.execute = True
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_shoot_client, goal_msg, "ball_shoot")

    async def send_ball_shoot_aim_goal(self, direction):
        goal_msg = BallShootAim.Goal()
        goal_msg.direction = direction
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_shoot_aim_client, goal_msg, "ball_shoot_aim")
        
    async def send_ball_gate_operation_goal(self, target_gate, is_open):
        goal_msg = BallGateOperation.Goal()
        goal_msg.target_gate = target_gate
        goal_msg.is_open = is_open
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_gate_operation_client, goal_msg, "ball_gate_operation")

    async def send_ball_intake_goal(self):
        goal_msg = BallIntake.Goal()
        goal_msg.execute = True
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_intake_client, goal_msg, "ball_intake")

    # ボタンが押された瞬間だけ取る
    def is_pressed(self,msg,idx):
        if idx < len(msg.buttons) and idx < len(self.prev_buttons):
            return msg.buttons[idx] == 1 and self.prev_buttons[idx] == 0
        return False

    # スティックが一定角度以上傾いたときだけとる
    def is_axis_changed(self,msg,idx, threshold, direction):
        if idx < len(msg.axes) and idx < len(self.prev_axes):
            if direction > 0:
                return msg.axes[idx] > threshold and self.prev_axes[idx] <= threshold
            else:
                return msg.axes[idx] < -threshold and self.prev_axes[idx] >= -threshold
        return False

    # joyスティックが入力されるたびに呼ばれるやつ
    async def joy_callback(self, msg):
        # 初期化だけする
        if not self.prev_buttons:
            self.prev_buttons = msg.buttons
            self.prev_axes = msg.axes
            return

        #各ボタン機構
        #OPTIONS: 強制リセット
        #◯: ボールを城門に入れる
        #□: ボールを射出
        #△: ボールを内側に取り込む
        #✕: ボールを皿の上に置く
        #L1: 左ゲートを開閉
        #R1: 右ゲートを開閉
        #L2: 左から取り込んで脇に抱える
        #R2: 右から取り込んで脇に抱える
        #十字キー上: 射出機構照準を上に向ける
        #十字キー下: 射出機構照準を下に向ける

        if self.is_pressed(msg,9): # OPTIONSボタンで強制リセット
            self.get_logger().info("OPTIONSボタン: 状態をUNKNOWNにリセットします")
            self.current_state = Mechanism_State.UNKNOWN
            self.is_action_running = False
            self.is_left_gate_open = False
            self.is_right_gate_open = False

        if self.is_pressed(msg,0): # ✕ボタン
            if not self.is_action_running:
                if self.current_state in [Mechanism_State.LEFT_CARRY, Mechanism_State.RIGHT_CARRY]:
                    self.get_logger().info("BallPutPlateが入力された")
                    self.is_action_running = True
                    try:
                        res = await self.send_ball_put_plate_goal()
                        if res and res.success:
                            self.current_state = Mechanism_State(res.next_state)
                            self.is_left_gate_open = False
                            self.is_right_gate_open = False
                    finally:
                        self.is_action_running = False
                else:
                    self.get_logger().warn('BallPutPlate は LEFT_CARRY または RIGHT_CARRY 状態でのみ許可されます')
            else:
                self.get_logger().warn('現在他の動作中なのむし')

        if self.is_pressed(msg,1): # ◯ボタン
            if not self.is_action_running:
                if self.current_state == Mechanism_State.INTAKE:
                    self.get_logger().info("BallPutGateが入力された")
                    self.is_action_running = True
                    try:
                        res = await self.send_ball_put_gate_goal()
                        if res and res.success:
                            self.current_state = Mechanism_State(res.next_state)
                    finally:
                        self.is_action_running = False
                else:
                    self.get_logger().warn('BallPutGate は INTAKE 状態でのみ許可されます')
            else:
                self.get_logger().warn('現在他の動作中なのむし')

        elif self.is_pressed(msg,2): # △ボタン
            if not self.is_action_running:
                if self.current_state in [Mechanism_State.LEFT_CARRY, Mechanism_State.RIGHT_CARRY]:
                    self.get_logger().info("BallIntakeが入力された")
                    self.is_action_running = True
                    try:
                        res = await self.send_ball_intake_goal() 
                        if res and res.success:
                            self.current_state = Mechanism_State(res.next_state)
                    finally:
                        self.is_action_running = False
                else:
                    self.get_logger().warn('BallIntake は LEFT_CARRY または RIGHT_CARRY 状態でのみ許可されます')
            else:
                self.get_logger().warn('現在他の動作中なのむし')

        elif self.is_pressed(msg,3): # □ボタン
            if not self.is_action_running:
                if self.current_state == Mechanism_State.INTAKE:
                    self.get_logger().info("BallShootが入力された")
                    self.is_action_running = True
                    try:
                        res = await self.send_ball_shoot_goal()
                        if res and res.success:
                            self.current_state = Mechanism_State(res.next_state)
                    finally:
                        self.is_action_running = False
                else:
                    self.get_logger().warn('BallShoot は INTAKE 状態でのみ許可されます')
            else:
                self.get_logger().warn('現在他の動作中なのむし')
        
        elif self.is_pressed(msg,4): # L1ボタン
            if not self.is_action_running:
                if self.current_state in [Mechanism_State.NOT_CARRY, Mechanism_State.UNKNOWN, Mechanism_State.LEFT_CARRY, Mechanism_State.INTAKE]:
                    self.get_logger().info("BallGateOperation(左)が入力された")
                    self.is_action_running = True
                    target_is_open = not self.is_left_gate_open
                    try:
                        res = await self.send_ball_gate_operation_goal(1, target_is_open)
                        if res and res.success:
                            self.is_left_gate_open = target_is_open
                            if self.current_state == Mechanism_State.LEFT_CARRY and self.is_left_gate_open:
                                self.get_logger().info("左脇のボールを排出したのでNOT_CARRYに戻します。")
                                self.current_state = Mechanism_State.NOT_CARRY
                            else:
                                self.current_state = Mechanism_State(res.next_state)
                    finally:
                        self.is_action_running = False
                else:
                    self.get_logger().warn('ゲートの手動開閉は現在の状態では許可されません')
            else:
                self.get_logger().warn('現在他の動作中なのむし')
        
        elif self.is_pressed(msg,5): # R1ボタン
            if not self.is_action_running:
                if self.current_state in [Mechanism_State.NOT_CARRY, Mechanism_State.UNKNOWN, Mechanism_State.RIGHT_CARRY, Mechanism_State.INTAKE]:
                    self.get_logger().info("BallGateOperation(右)が入力された")
                    self.is_action_running = True
                    target_is_open = not self.is_right_gate_open
                    try:
                        res = await self.send_ball_gate_operation_goal(2, target_is_open)
                        if res and res.success:
                            self.is_right_gate_open = target_is_open
                            if self.current_state == Mechanism_State.RIGHT_CARRY and self.is_right_gate_open:
                                self.get_logger().info("右脇のボールを排出したのでNOT_CARRYに戻します。")
                                self.current_state = Mechanism_State.NOT_CARRY
                            else:
                                self.current_state = Mechanism_State(res.next_state)
                    finally:
                        self.is_action_running = False
                else:
                    self.get_logger().warn('ゲートの手動開閉は現在の状態では許可されません')
            else:
                self.get_logger().warn('現在他の動作中なのむし')
        
        elif self.is_pressed(msg,6): # L2ボタン
            if not self.is_action_running:
                if self.current_state in [Mechanism_State.NOT_CARRY, Mechanism_State.UNKNOWN]:
                    self.get_logger().info("BallGet(左)が入力された")
                    self.is_action_running = True
                    try:
                        res = await self.send_ball_get_goal(1)
                        if res and res.success:
                            self.current_state = Mechanism_State(res.next_state)
                            self.is_left_gate_open = False
                    finally:
                        self.is_action_running = False
                else:
                    self.get_logger().warn('BallGet は NOT_CARRY または UNKNOWN 状態でのみ許可されます')
            else:
                self.get_logger().warn('現在他の動作中なのむし')
        
        elif self.is_pressed(msg,7): # R2ボタン
            if not self.is_action_running:
                if self.current_state in [Mechanism_State.NOT_CARRY, Mechanism_State.UNKNOWN]:
                    self.get_logger().info("BallGet(右)が入力された")
                    self.is_action_running = True
                    try:
                        res = await self.send_ball_get_goal(2)
                        if res and res.success:
                            self.current_state = Mechanism_State(res.next_state)
                            self.is_right_gate_open = False
                    finally:
                        self.is_action_running = False
                else:
                    self.get_logger().warn('BallGet は NOT_CARRY または UNKNOWN 状態でのみ許可されます')
            else:
                self.get_logger().warn('現在他の動作中なのむし')

        elif self.is_axis_changed(msg, 7, 0.5, 1): # 十字キー上
            if not self.is_action_running:
                self.get_logger().info("十字キー上が押されました(射出機構 上)")
                self.is_action_running = True
                try:
                    res = await self.send_ball_shoot_aim_goal(1)
                    if res and res.success:
                        self.current_state = Mechanism_State(res.next_state)
                finally:
                    self.is_action_running = False
            else:
                self.get_logger().warn('現在他の動作中なのむし')

        elif self.is_axis_changed(msg, 7, 0.5, -1): # 十字キー下
            if not self.is_action_running:
                self.get_logger().info("十字キー下が押されました(射出機構 下)")
                self.is_action_running = True
                try:
                    res = await self.send_ball_shoot_aim_goal(-1)
                    if res and res.success:
                        self.current_state = Mechanism_State(res.next_state)
                finally:
                    self.is_action_running = False
            else:
                self.get_logger().warn('現在他の動作中なのむし')

        self.prev_buttons = msg.buttons
        self.prev_axes = msg.axes

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

if __name__ == '__main__':
    main()
