import rclpy
from rclpy.node import Node

class SimpleNode(Node):
    def __init__(self):
        # ノード名を 'simple_node' として初期化
        super().__init__('simple_node')
        self.get_logger().info('ノードが正常に起動しました！')

def main(args=None):
    rclpy.init(args=args)
    node = SimpleNode()
    
    # ノードを待機状態にする
    rclpy.spin(node)
    
    # 終了処理
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()
