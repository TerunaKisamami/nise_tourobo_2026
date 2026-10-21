import rclpy
from rclpy.node import Node
from enum import Enum, auto

# 仮想的なインターフェースのインポート例 (実際のものに書き換えてください)
# from tourobo_2026_interfaces.action import BallGet
# from rclpy.action import ActionClient

#自動機の状態遷移を制御するプログラム

#各状態
class StrategyState(Enum):
    #初期
    INIT = auto()
    
    SEARCHING_BALL = auto()
    MOVING_TO_BALL = auto()
    GETTING_BALL = auto()
    READY_TO_SHOOT = auto()

class MainStrategyNode(Node):
    def __init__(self):
        super().__init__('main_strategy_node')
        self.get_logger().info('自動制御(戦略)ノードが起動しました。')
        
        # 現在の状態を保持
        self.current_state = StrategyState.INIT
        
        # 状態遷移を管理するタイマー（ここでは 0.1秒 = 10Hz で回す例）
        self.timer = self.create_timer(0.1, self.strategy_loop)
        
        # 必要なAction ClientやSubscriber、Publisherを初期化する
        # self.ball_get_client = ActionClient(self, BallGet, 'ball_get')
        # self.camera_sub = self.create_subscription(...)

    def strategy_loop(self):
        """
        一定周期で呼ばれ、現在の状態に応じた処理と状態遷移（ステートマシン）を行う
        """
        if self.current_state == StrategyState.INIT:
            # throttle_duration_secを指定すると、毎回出力されず指定秒数に1回だけ出力されます(ログのスパム防止)
            self.get_logger().info('状態: 初期化処理中...', throttle_duration_sec=2.0)
            
            # 初期化(各種センサーの起動待ちなど)が完了したと仮定して次の状態へ
            self.current_state = StrategyState.SEARCHING_BALL
            
        elif self.current_state == StrategyState.SEARCHING_BALL:
            self.get_logger().info('状態: ボールを探索中...', throttle_duration_sec=2.0)
            
            # TODO: センサーやカメラからの情報(Subscriber)を元に判断する
            # 例:
            # if self.is_ball_found:
            #     self.current_state = StrategyState.MOVING_TO_BALL
            
        elif self.current_state == StrategyState.MOVING_TO_BALL:
            self.get_logger().info('状態: ボールへ移動中...', throttle_duration_sec=2.0)
            # TODO: モーター駆動系への目標値パブリッシュなど
            
        elif self.current_state == StrategyState.GETTING_BALL:
            self.get_logger().info('状態: ボール回収アクション実行中...', throttle_duration_sec=2.0)
            # TODO: Actionを投げて、完了結果が返ってくるのを待つ

def main(args=None):
    rclpy.init(args=args)
    node = MainStrategyNode()
    
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()
