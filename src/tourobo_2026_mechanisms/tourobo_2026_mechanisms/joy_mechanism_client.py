import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from rclpy.callback_groups import ReentrantCallbackGroup
from sensor_msgs.msg import Joy
from action_msgs.msg import GoalStatus
from enum import Enum

from tourobo_2026_interfaces.action import BallGet, BallPutGate, BallPutPlate, BallShoot, BallShootAim, BallArmOperation, BallIntake


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
    INTAKE_GATE = 4
    INTAKE_SHOOT = 5

class Shoot_Push_State(Enum):
    MIN=0
    GATE_HOLD =1  #城門に設置するまえにボールを支える角度(仮)
    LOADING =2 #射出機構に装填するときの角度(仮)
    MAX = 3 #射出機構に近いほどでかい



class JoyMechanismClient(Node):

    def __init__(self):
        super().__init__('joy_mechanism_client')
        self.cb_group = ReentrantCallbackGroup()

        self.joy_sub = self.create_subscription(Joy,
                                                '/joy',
                                                self.joy_callback,
                                                10,
                                                callback_group=self.cb_group)

        # 論理状態の初期化
        self.current_state = Mechanism_State.UNKNOWN
        self.shoot_push_state = Shoot_Push_State.MIN
        self.is_left_arm_open = False
        self.is_right_arm_open = False
        self.push_state = Push_State.MIN

        # クライアント設定
        self.ball_get_client = ActionClient(self,
                                            BallGet,
                                            'ball_get',
                                            callback_group=self.cb_group)
        self.ball_put_gate_client = ActionClient(self,
                                                 BallPutGate,
                                                 'ball_put_gate',
                                                 callback_group=self.cb_group)
        self.ball_put_plate_client = ActionClient(self,
                                                  BallPutPlate,
                                                  'ball_put_plate',
                                                  callback_group=self.cb_group)
        self.ball_shoot_client = ActionClient(self,
                                              BallShoot,
                                              'ball_shoot',
                                              callback_group=self.cb_group)
        self.ball_shoot_aim_client = ActionClient(self,
                                                  BallShootAim,
                                                  'ball_shoot_aim',
                                                  callback_group=self.cb_group)
        self.ball_arm_operation_client = ActionClient(
            self,
            BallArmOperation,
            'ball_arm_operation',
            callback_group=self.cb_group)
        self.ball_intake_client = ActionClient(self,
                                               BallIntake,
                                               'ball_intake',
                                               callback_group=self.cb_group)

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

        self.get_logger().info(
            f"[{action_name}] ゴール送信 (current_state: {self.current_state.name})")
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
                f"[{action_name}] 失敗しました (Status ID: {result_handle.status})")
            return None

    # 各種アクションのゴール生成
    async def send_ball_get_goal(self, execute_mode):
        goal_msg = BallGet.Goal()
        goal_msg.execute_mode = execute_mode
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_get_client, goal_msg,
                                           "ball_get")

    async def send_ball_intake_goal(self, execute_mode):
        goal_msg = BallIntake.Goal()
        goal_msg.push_state = self.push_state.value
        goal_msg.execute_mode = execute_mode
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_intake_client, goal_msg,
                                           "ball_intake")

    async def send_ball_put_gate_goal(self):
        goal_msg = BallPutGate.Goal()
        goal_msg.push_state = self.push_state.value
        goal_msg.execute = True
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_put_gate_client, goal_msg,
                                           "ball_put_gate")

    async def send_ball_put_plate_goal(self):
        goal_msg = BallPutPlate.Goal()
        goal_msg.push_state = self.push_state.value
        goal_msg.execute = True
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_put_plate_client, goal_msg,
                                           "ball_put_plate")

    async def send_ball_shoot_goal(self):
        goal_msg = BallShoot.Goal()
        goal_msg.push_state = self.push_state.value
        goal_msg.execute = True
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_shoot_client, goal_msg,
                                           "ball_shoot")

    async def send_ball_arm_operation_goal(self, target_arm, is_open):
        goal_msg = BallArmOperation.Goal()
        goal_msg.target_arm = target_arm
        goal_msg.is_open = is_open
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_arm_operation_client, goal_msg,
                                           "ball_arm_operation")

    async def send_ball_shoot_aim_goal(self, direction):
        goal_msg = BallShootAim.Goal()
        goal_msg.direction = direction
        goal_msg.current_state = self.current_state.value
        return await self.send_action_goal(self.ball_shoot_aim_client, goal_msg,
                                           "ball_shoot_aim")

    # ボタンが押された瞬間だけ取る
    def is_pressed(self, msg, idx):
        if idx < len(msg.buttons) and idx < len(self.prev_buttons):
            return msg.buttons[idx] == 1 and self.prev_buttons[idx] == 0
        return False

    # スティックが一定角度以上傾いたときだけとる
    def is_axis_changed(self, msg, idx, threshold, direction):
        if idx < len(msg.axes) and idx < len(self.prev_axes):
            if direction > 0:
                return msg.axes[idx] > threshold and self.prev_axes[
                    idx] <= threshold
            else:
                return msg.axes[idx] < -threshold and self.prev_axes[
                    idx] >= -threshold
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
        #□: ボールを内側に取り込む(射撃用)→もう一度押すとボールを射出
        #△: ボールを内側に取り込む(城門用)→もう一度押すと城門に設置
        #◯: 今抱えている方向の反対側へボールを関所に配置
        #L1: 左ゲートを開閉
        #R1: 右ゲートを開閉
        #L2: 左から脇に抱える
        #R2: 右から脇に抱える
        #十字キー上: 射出機構照準を上に向ける
        #十字キー下: 射出機構照準を下に向ける


        if self.is_pressed(msg, 9):  # OPTIONSボタンで強制リセット
            self.get_logger().info("OPTIONSボタン: 状態をUNKNOWNにリセットします")
            self.current_state = Mechanism_State.UNKNOWN
            self.is_action_running = False
            self.is_left_arm_open = False
            self.is_right_arm_open = False
            self.push_state = Push_State.MIN
            
        elif self.is_action_running:
            # 動作中は何のボタンを押しても受け付けない
            # 何かボタンや軸が操作されたときだけ警告を出す
            if any(self.is_pressed(msg, b) for b in [2, 3, 4, 5, 6, 7]) or \
               self.is_axis_changed(msg, 7, 0.5, 1) or self.is_axis_changed(msg, 7, 0.5, -1):
                self.get_logger().warn('現在他の動作中です')


        # △: ボールを城門に入れる
        elif self.is_pressed(msg, 2):

            self.get_logger().info(f"current_state = {self.current_state.name}")
            self.get_logger().info("△ボタンが入力された")

            #current_stateがLEFT_CARRYかRIGHT_CARRYならば城門用内側取り込み
            if self.current_state in [Mechanism_State.LEFT_CARRY, Mechanism_State.RIGHT_CARRY]:
                self.is_action_running = True
                res = await self.send_ball_intake_goal(1)

                if res and res.success:
                    self.current_state = Mechanism_State(res.next_state)
                    self.push_state = Push_State.MAX
                self.is_action_running = False

            elif self.current_state == Mechanism_State.INTAKE_GATE:
                self.is_action_running = True
                res = await self.send_ball_put_gate_goal()

                if res and res.success:
                    self.current_state = Mechanism_State(res.next_state)
                    self.push_state = Push_State.MIN
                self.is_action_running = False

 

        # □: 射出用取り込み + ボールを射出
        elif self.is_pressed(msg, 3):

            self.get_logger().info(f"current_state ={self.current_state.name}")
            self.get_logger().info("□ボタンが入力された")

            #current_stateがLEFT_CARRYかRIGHT_CARRYならば内側取り込み
            if self.current_state in [Mechanism_State.LEFT_CARRY, Mechanism_State.RIGHT_CARRY]:
                self.is_action_running = True
                res = await self.send_ball_intake_goal()

                if res and res.success:
                   self.current_state = Mechanism_State(res.next_state)
                self.is_action_running = False

            #current_stateがならば内側取り込み
            elif self.current_state == Mechanism_State.INTAKE_SHOOT:
                self.is_action_running = True
                res = await self.send_ball_shoot_goal()

                if res and res.success:
                    self.current_state = Mechanism_State(res.next_state)
                    self.push_state = Push_State.MAX
                self.is_action_running= False


        # L1: 左ゲートを開閉
        elif self.is_pressed(msg, 4):

            self.get_logger().info("L1が入力された")
            self.is_action_running = True
            target_is_open = not self.is_left_arm_open
            res = await self.send_ball_arm_operation_goal(1, target_is_open)

            if res and res.success:
                self.is_left_arm_open = target_is_open
                if self.current_state == Mechanism_State.LEFT_CARRY and self.is_left_arm_open:
                    self.current_state = Mechanism_State.NOT_CARRY
                else:
                    self.current_state = Mechanism_State(res.next_state)
            self.is_action_running = False


        # R1: 右ゲートを開閉
        elif self.is_pressed(msg, 5):

            self.get_logger().info("R1が入力された")
            self.is_action_running = True
            target_is_open = not self.is_right_arm_open
            res = await self.send_ball_arm_operation_goal(2, target_is_open)
            if res and res.success:
                self.is_right_arm_open = target_is_open
                if self.current_state == Mechanism_State.RIGHT_CARRY and self.is_right_arm_open:
                    self.current_state = Mechanism_State.NOT_CARRY
                else:
                    self.current_state = Mechanism_State(res.next_state)
            self.is_action_running = False

        # L2: 左から脇に抱える
        elif self.is_pressed(msg, 6):

            if not self.is_left_arm_open:
                self.get_logger().warn('左アームが開いていないため、BallGet(左)は実行できません')
                return
                
            if self.current_state in [Mechanism_State.NOT_CARRY, Mechanism_State.UNKNOWN]:
                self.get_logger().info("L2が入力された")
                self.is_action_running = True
                res = await self.send_ball_get_goal(1)
                if res and res.success:
                    self.current_state = Mechanism_State(res.next_state)
                    self.is_left_arm_open = False
                self.is_action_running = False

        #R2: 右から脇に抱える
        elif self.is_pressed(msg, 7):
            
            if not self.is_right_arm_open:
                self.get_logger().warn('右アームが開いていないため、BallGet(右)は実行できません')
                return
            
            if self.current_state in [Mechanism_State.NOT_CARRY, Mechanism_State.UNKNOWN]:
                self.get_logger().info('R2が入力された')
                self.is_action_running = True
                res = await self.send_ball_get_goal(2)
                if res and res.success:
                    self.current_state = Mechanism_State(res.next_state)
                    self.is_right_arm_open = False
                self.is_action_running = False
            

        # 十字キー上: 射出機構照準を上に向ける
        elif self.is_axis_changed(msg, 7, 0.5, 1):
            if self.current_state == Mechanism_State.INTAKE_SHOOT:
                self.get_logger().info("十字キー上が押されました(射出機構照準 上)")
                self.is_action_running = True
                res = await self.send_ball_shoot_aim_goal(1)
                if res and res.success:
                    self.current_state = Mechanism_State(res.next_state)
                self.is_action_running = False
            else:
                self.get_logger().warn("射出機構の照準操作(上)は INTAKE_SHOOT 状態でのみ許可されます")

        # 十字キー下: 射出機構照準を下に向ける
        elif self.is_axis_changed(msg, 7, 0.5, -1):
            if self.current_state == Mechanism_State.INTAKE_SHOOT:
                self.get_logger().info("十字キー下が押されました(射出機構照準 下)")
                self.is_action_running = True
                res = await self.send_ball_shoot_aim_goal(-1)
                if res and res.success:
                    self.current_state = Mechanism_State(res.next_state)
                self.is_action_running = False
            else:
                self.get_logger().warn("射出機構の照準操作(下)は INTAKE_SHOOT 状態でのみ許可されます")

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
