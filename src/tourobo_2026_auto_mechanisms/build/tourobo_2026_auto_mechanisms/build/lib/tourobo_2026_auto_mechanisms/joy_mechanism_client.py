import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from rclpy.callback_groups import ReentrantCallbackGroup
from sensor_msgs.msg import Joy

# 独自のインターフェース
from tourobo_2026_interfaces.action import BallGet

class JoyMechanismClient(Node):

    def __init__(self):
        super().__init__('joy_mechanism_client')
        self.cb_group = ReentrantCallbackGroup()

        # サブスクライバー
        self.joy_sub = self.create_subscription(
            Joy,
            '/joy',
            self.joy_callback,
            10,
            callback_group=self.cb_group
        )

        # アクションクライアント
        self.ball_get_client = ActionClient(self, BallGet, 'ball_get', callback_group=self.cb_group)

        # 前回のボタン・軸状態を保持
        self.prev_buttons = []
        self.prev_axes = []
        
        # 実行中のフラグ (連打防止)
        self.is_action_running = False

    def joy_callback(self, msg):
        if not self.prev_buttons:
            self.prev_buttons = msg.buttons
            self.prev_axes = msg.axes
            return

        # エッジ検出用
        def pressed(idx):
            if idx < len(msg.buttons) and idx < len(self.prev_buttons):
                return msg.buttons[idx] == 1 and self.prev_buttons[idx] == 0
            return False

        def axis_changed(idx, threshold, direction):
            if idx < len(msg.axes) and idx < len(self.prev_axes):
                if direction > 0:
                    return msg.axes[idx] > threshold and self.prev_axes[idx] <= threshold
                else:
                    return msg.axes[idx] < -threshold and self.prev_axes[idx] >= -threshold
            return False

        if not self.is_action_running:
            # マッピング例 (PS4)
            # D-pad Down(axes[7] < -0.5)   : BallGet
                self.send_ball_get_goal()

        self.prev_buttons = msg.buttons
        self.prev_axes = msg.axes

    def send_ball_get_goal(self):
        self.get_logger().info("Sending BallGet Goal...")
        if not self.ball_get_client.wait_for_server(timeout_sec=1.0):
            self.get_logger().error("BallGet action server not available!")
            return
        
        goal_msg = BallGet.Goal()
        goal_msg.mode = 0
        self.is_action_running = True
        self.ball_get_client.send_goal_async(goal_msg).add_done_callback(self.goal_response_callback)

    def goal_response_callback(self, future):
        goal_handle = future.result()
        if not goal_handle.accepted:
            self.get_logger().warn("Goal rejected!")
            self.is_action_running = False
            return

        self.get_logger().info("Goal accepted. Waiting for result...")
        goal_handle.get_result_async().add_done_callback(self.get_result_callback)

    def get_result_callback(self, future):
        result = future.result().result
        self.get_logger().info(f"Action finished with success: {result.success}")
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

if __name__ == '__main__':
    main()
