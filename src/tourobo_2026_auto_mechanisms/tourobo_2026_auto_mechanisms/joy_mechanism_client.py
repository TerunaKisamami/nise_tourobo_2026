import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from rclpy.callback_groups import ReentrantCallbackGroup
from sensor_msgs.msg import Joy
from action_msgs.msg import GoalStatus

from tourobo_2026_interfaces.action import BallGet, BallPutGate, BallPutPlate, BallShoot

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

        #クライアント設定
        self.ball_get_client = ActionClient(self, BallGet, 'ball_get', callback_group=self.cb_group)
        self.ball_put_gate_client = ActionClient(self, BallPutGate, 'ball_put_gate', callback_group=self.cb_group)
        self.ball_put_plate_client = ActionClient(self, BallPutPlate, 'ball_put_plate', callback_group=self.cb_group)
        self.ball_shoot_client = ActionClient(self, BallShoot, 'ball_shoot', callback_group=self.cb_group)

        #ボタンの状態保持用
        self.prev_buttons = []
        self.prev_axes = []
        
        #連打防止用のフラグ
        self.is_action_running = False

    #action送信
    async def send_action_goal(self, client, goal_msg, action_name):
        self.get_logger().info(f"[{action_name}] サーバーを待機中...")
        if not client.wait_for_server(timeout_sec=1.0):
            self.get_logger().error(f"[{action_name}] サーバーが見つかりません")
            return False

        self.get_logger().info(f"[{action_name}] ゴール送信開始")
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

    #joyスティックが入力されるたびに呼ばれるやつ
    async def joy_callback(self, msg):
        #初期化だけする
        if not self.prev_buttons:
            self.prev_buttons = msg.buttons
            self.prev_axes = msg.axes
            return

        #ボタンが押された瞬間だけ取る
        def pressed(idx):
            if idx < len(msg.buttons) and idx < len(self.prev_buttons):
                return msg.buttons[idx] == 1 and self.prev_buttons[idx] == 0
            return False

        #スティックが一定角度以上傾いたときだけとる
        def axis_changed(idx, threshold, direction):
            if idx < len(msg.axes) and idx < len(self.prev_axes):
                if direction > 0:
                    return msg.axes[idx] > threshold and self.prev_axes[idx] <= threshold
                else:
                    return msg.axes[idx] < -threshold and self.prev_axes[idx] >= -threshold
            return False

        # マッピング (PS4コン)
        # D-pad Down(axes[7] < -0.5) : BallGet(ボール取得)
        # D-pad Up(axes[7] > 0.5)    : BallPutGate(ボールを城門に置く)
        # D-pad Right(axes[6] < -0.5): BallPutPlate(ボールを皿の上に置く)
        # D-pad Left(axes[6] > 0.5)  : BallShoot(ボール発射)
        
        target_coroutine = None
        action_name_log = ""

        if axis_changed(7, 0.5, -1):
            target_coroutine = self.send_ball_get_goal
            action_name_log = "BallGet"
        elif axis_changed(7, 0.5, 1):
            target_coroutine = self.send_ball_put_gate_goal
            action_name_log = "BallPutGate"
        elif axis_changed(6, 0.5, -1):
            target_coroutine = self.send_ball_put_plate_goal
            action_name_log = "BallPutPlate"
        elif axis_changed(6, 0.5, 1):
            target_coroutine = self.send_ball_shoot_goal
            action_name_log = "BallShoot"

        #選ばれた動作を実行するのじゃ
        if target_coroutine:
            if not self.is_action_running:
                self.get_logger().info(f'{action_name_log} が入力された')
                self.is_action_running = True
                try:
                    await target_coroutine()
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
