from numpy.core import einsumfunc
import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from rclpy.callback_groups import ReentrantCallbackGroup
from sensor_msgs.msg import Joy
from action_msgs.msg import GoalStatus
from enum import Enum
# pyrefly: ignore [missing-import]
from dyna_interfaces.msg import DynaFeedback

from tourobo_2026_interfaces.action import BallGet, BallPutGate, BallPutPlate, BallShoot, BallShootAim

class Mechanism_State(Enum):
    UNKNOWN = 0
    #でふぉると
    NOT_CARRY = 1

    #ボールを脇に保持
    LEFT_CARRY = 2
    RIGHT_CARRY = 3

    #
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

        # フィードバック受信用（物理的な状態確認のため）
        self.current_dyna_pos = {}
        self.dyna_feedback_sub = self.create_subscription(
            DynaFeedback,
            '/dyna_feedback',
            self.dyna_feedback_callback,
            10,
            callback_group=self.cb_group
        )

        #クライアント設定
        self.ball_get_client = ActionClient(self, BallGet, 'ball_get', callback_group=self.cb_group)
        self.ball_put_gate_client = ActionClient(self, BallPutGate, 'ball_put_gate', callback_group=self.cb_group)
        self.ball_put_plate_client = ActionClient(self, BallPutPlate, 'ball_put_plate', callback_group=self.cb_group)
        self.ball_shoot_client = ActionClient(self, BallShoot, 'ball_shoot', callback_group=self.cb_group)
        self.ball_shoot_aim_client = ActionClient(self, BallShootAim, 'ball_shoot_aim', callback_group=self.cb_group)

        #ボタンの状態保持用
        self.prev_buttons = []
        self.prev_axes = []
        
        #連打防止用のフラグ
        self.is_action_running = False

    def dyna_feedback_callback(self, msg):
        # 常に最新の角度データを辞書に保存する
        self.current_dyna_pos[msg.id] = msg.data[0]

    #action送信
    async def send_action_goal(self, client, goal_msg, action_name):
        self.get_logger().info(f"[{action_name}] サーバーを待機中...")
        if not client.wait_for_server(timeout_sec=1.0):
            self.get_logger().error(f"[{action_name}] サーバーが見つかりません")
            return False

        self.get_logger().info(f"[{action_name}] ゴール送信")
        send_goal_future = await client.send_goal_async(goal_msg)

        if not send_goal_future.accepted:
            self.get_logger().error(f"[{action_name}] 命令が拒否されました")
            return False

        self.get_logger().info(f"[{action_name}] 受理されました。結果を待機します...")
        result_handle = await send_goal_future.get_result_async()

        if result_handle.status == GoalStatus.STATUS_SUCCEEDED:
            self.get_logger().info(f"[{action_name}] 正常完了しました")
            return result_handle.result
        else:
            self.get_logger().warn(f"[{action_name}] 失敗しました (Status ID: {result_handle.status})")
            return None

    async def send_ball_get_goal(self):
        goal_msg = BallGet.Goal()
        goal_msg.execute = True
        return await self.send_action_goal(self.ball_get_client, goal_msg, "ball_get")

    async def send_ball_put_gate_goal(self):
        goal_msg = BallPutGate.Goal()
        goal_msg.execute = True
        return await self.send_action_goal(self.ball_put_gate_client, goal_msg, "ball_put_gate")

    async def send_ball_put_plate_goal(self):
        goal_msg = BallPutPlate.Goal()
        goal_msg.execute = True
        return await self.send_action_goal(self.ball_put_plate_client, goal_msg, "ball_put_plate")

    async def send_ball_shoot_goal(self):
        goal_msg = BallShoot.Goal()
        goal_msg.execute = True
        return await self.send_action_goal(self.ball_shoot_client, goal_msg, "ball_shoot")

    async def send_ball_shoot_aim_goal(self, direction):
        goal_msg = BallShootAim.Goal()
        goal_msg.direction = direction
        return await self.send_action_goal(self.ball_shoot_aim_client, goal_msg, "ball_shoot_aim")

    #ボタンが押された瞬間だけ取る
    def is_pressed(self,msg,idx):
        if idx < len(msg.buttons) and idx < len(self.prev_buttons):
            return msg.buttons[idx] == 1 and self.prev_buttons[idx] == 0
        return False

    #スティックが一定角度以上傾いたときだけとる
    def is_axis_changed(self,msg,idx, threshold, direction):
        if idx < len(msg.axes) and idx < len(self.prev_axes):
            if direction > 0:
                return msg.axes[idx] > threshold and self.prev_axes[idx] <= threshold
            else:
                return msg.axes[idx] < -threshold and self.prev_axes[idx] >= -threshold
        return False

    #joyスティックが入力されるたびに呼ばれるやつ
    async def joy_callback(self, msg):
        #初期化だけする
        if not self.prev_buttons:
            self.prev_buttons = msg.buttons
            self.prev_axes = msg.axes
            return


        # マッピング (PS4コン)
        # ✕ボタン (buttons[0]): ボールを脇に抱えているならば反対側から関所へ運ぶ
        # ◯ボタン (buttons[1]): ボールを内側に保持しているならば城門に置く
        # △ボタン (buttons[2]): ボールを脇に抱えているならば内側へ移動
        # □ボタン (buttons[3]): ボールを内側に保持しているならば発射
        # L1ボタン (buttons[4]): 左ゲートの開閉
        # R1ボタン (buttons[5]): 右ゲートの開閉
        # L2ボタン (buttons[6]): ボールを左から取得し脇にかかえる
        # R2ボタン (buttons[7]): ボールを右から取得し脇にかかえる
        #十字キー上 : 射出機構を上にむける
        #十字キー下 : 射出機構を下に向ける

        if self.is_pressed(msg,0): # ✕ボタン
            if not self.is_action_running:
                self.get_logger().info("BallPutPlateが入力された")
                self.is_action_running = True
                try:
                    await self.send_ball_put_plate_goal()
                finally:
                    self.is_action_running = False
            else:
                self.get_logger().warn('現在他の動作中なのむし')
            

        if self.is_pressed(msg,1): # ◯ボタン
            if not self.is_action_running:
                self.get_logger().info("BallPutGateが入力された")
                self.is_action_running = True
                try:
                    await self.send_ball_put_gate_goal()
                finally:
                    self.is_action_running = False
            else:
                self.get_logger().warn('現在他の動作中なのむし')

        elif self.is_pressed(msg,2): # △ボタン
            if not self.is_action_running:
                self.get_logger().info("BallIntakeが入力された")
                self.is_action_running = True
                try:
                    await self.send_ball_intake_goal() 
                finally:
                    self.is_action_running = False
            else:
                self.get_logger().warn('現在他の動作中なのむし')

        elif self.is_pressed(msg,3): # □ボタン
            if not self.is_action_running:
                self.get_logger().info("BallShootが入力された")
                self.is_action_running = True
                try:
                    await self.send_ball_shoot_goal()
                finally:
                    self.is_action_running = False
            else:
                self.get_logger().warn('現在他の動作中なのむし')
        
        elif self.is_pressed(msg,4): # L1ボタン
            if not self.is_action_running:
                self.get_logger().info("BallGateOperation(左ゲートの開閉)が入力された")
                self.is_action_running = True
                try:
                    await self.send_ball_gate_operation_goal(1)
                finally:
                    self.is_action_running = False
            else:
                self.get_logger().warn('現在他の動作中なのむし')
        
        elif self.is_pressed(msg,5): # R1ボタン
            if not self.is_action_running:
                self.get_logger().info("BallGateOperation(右ゲートの開閉)が入力された")
                self.is_action_running = True
                try:
                    await self.send_ball_gate_operation_goal(2)
                finally:
                    self.is_action_running = False
            else:
                self.get_logger().warn('現在他の動作中なのむし')
        
        elif self.is_pressed(msg,6): # L2ボタン
            if not self.is_action_running:
                self.get_logger().info("BallGet(左から回収)が入力された")
                self.is_action_running = True
                try:
                    await self.send_ball_get_goal(1)
                finally:
                    self.is_action_running = False
            else:
                self.get_logger().warn('現在他の動作中なのむし')
        
        elif self.is_pressed(msg,7): # R2ボタン
            if not self.is_action_running:
                self.get_logger().info("BallGet(右から回収)が入力された")
                self.is_action_running = True
                try:
                    await self.send_ball_get_goal(2)
                finally:
                    self.is_action_running = False
            else:
                self.get_logger().warn('現在他の動作中なのむし')

        elif self.is_axis_changed(msg, 7, 0.5, 1): # 十字キー上
            if not self.is_action_running:
                self.get_logger().info("十字キー上が押されました(射出機構 上)")
                self.is_action_running = True
                try:
                    await self.send_ball_shoot_aim_goal(1)
                finally:
                    self.is_action_running = False
            else:
                self.get_logger().warn('現在他の動作中なのむし')

        elif self.is_axis_changed(msg, 7, 0.5, -1): # 十字キー下
            if not self.is_action_running:
                self.get_logger().info("十字キー下が押されました(射出機構 下)")
                self.is_action_running = True
                try:
                    await self.send_ball_shoot_aim_goal(-1)
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
