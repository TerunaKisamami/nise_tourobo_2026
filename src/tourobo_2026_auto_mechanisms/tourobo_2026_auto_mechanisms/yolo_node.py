#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from std_msgs.msg import String

import pyrealsense2 as rs
import numpy as np
import json
import sys
import os

# yolo_detector.py があるディレクトリ（yolo_tourobo）にパスを通す
YOLO_DIR = os.path.expanduser('~/Robobobo/yolo_tourobo')
if YOLO_DIR not in sys.path:
    sys.path.append(YOLO_DIR)

try:
    from yolo_detector import YoloDetector
except ImportError as e:
    print(f"yolo_detectorのインポートに失敗しました。パスを確認してください: {e}")
    sys.exit(1)

class YoloRealSenseNode(Node):
    def __init__(self):
        super().__init__('yolo_realsense_node')
        
        # ROS2パラメータの宣言（起動時に上書き可能）
        self.declare_parameter('model_path', os.path.join(YOLO_DIR, 'yolo_assets/robocon_models/custom_model_v1/weights/best.pt'))
        self.declare_parameter('conf_threshold', 0.5)
        self.declare_parameter('fps', 30.0)

        model_path = self.get_parameter('model_path').value
        conf_threshold = self.get_parameter('conf_threshold').value
        fps = self.get_parameter('fps').value

        self.get_logger().info(f"モデルをロード中: {model_path}")
        self.detector = YoloDetector(model_path=model_path, conf_threshold=conf_threshold)
        
        # JSON形式の検出結果をパブリッシュするPublisherの設定
        self.publisher_ = self.create_publisher(String, '/yolo/detections', 10)
        
        # RealSenseの初期化
        self.pipeline = rs.pipeline()
        config = rs.config()
        # TH50の負荷を考慮した解像度設定
        config.enable_stream(rs.stream.color, 640, 480, rs.format.bgr8, 30)
        config.enable_stream(rs.stream.depth, 640, 480, rs.format.z16, 30)
        
        self.align_to = rs.stream.color
        self.align = rs.align(self.align_to)
        
        # デプス欠損対策用のフィルター群
        self.spatial_filter = rs.spatial_filter()
        self.temporal_filter = rs.temporal_filter()
        self.hole_filling_filter = rs.hole_filling_filter()
        
        try:
            self.pipeline.start(config)
            self.get_logger().info("RealSenseを起動しました。")
        except Exception as e:
            self.get_logger().error(f"RealSenseの起動に失敗しました: {e}")
            sys.exit(1)

        # 指定したFPSで定期的に推論・パブリッシュを実行
        timer_period = 1.0 / fps
        self.timer = self.create_timer(timer_period, self.timer_callback)

    def timer_callback(self):
        frames = self.pipeline.wait_for_frames()
        aligned_frames = self.align.process(frames)
        color_frame = aligned_frames.get_color_frame()
        depth_frame = aligned_frames.get_depth_frame()

        if not color_frame or not depth_frame:
            return

        # デプスフレームにフィルターを適用して精度向上
        depth_frame = self.spatial_filter.process(depth_frame)
        depth_frame = self.temporal_filter.process(depth_frame)
        depth_frame = self.hole_filling_filter.process(depth_frame).as_depth_frame()

        color_image = np.asanyarray(color_frame.get_data())

        # YOLOによる推論（画像への描画処理は不要なので破棄）
        _, detections = self.detector.detect(color_image)
        
        depth_intrin = depth_frame.profile.as_video_stream_profile().intrinsics
        
        results = []
        for det in detections:
            x1, y1, x2, y2 = det['bbox']
            cx = int((x1 + x2) / 2)
            cy = int((y1 + y2) / 2)
            
            # インデックスの範囲外アクセスを防ぐ
            cx = max(0, min(cx, color_frame.width - 1))
            cy = max(0, min(cy, color_frame.height - 1))

            # 中心点周辺から有効な距離を取得
            half_w = 2
            valid_distances = []
            for dy in range(-half_w, half_w + 1):
                for dx in range(-half_w, half_w + 1):
                    px, py = cx + dx, cy + dy
                    if 0 <= px < color_frame.width and 0 <= py < color_frame.height:
                        dist = depth_frame.get_distance(px, py)
                        if dist > 0:
                            valid_distances.append(dist)
            
            if len(valid_distances) > 0:
                distance = float(np.median(valid_distances))
                x_val, y_val, z_val = rs.rs2_deproject_pixel_to_point(depth_intrin, [cx, cy], distance)
                
                # ROS側で扱いやすいように辞書型に格納
                results.append({
                    "class": det['class_name'],
                    "confidence": float(det['confidence']),
                    "x": float(x_val),  # メートル単位
                    "y": float(y_val),  # メートル単位
                    "z": float(z_val),  # メートル単位
                    "distance": distance
                })

        # JSON文字列に変換してPublish
        msg = String()
        msg.data = json.dumps(results)
        self.publisher_.publish(msg)

    def destroy_node(self):
        self.pipeline.stop()
        super().destroy_node()

def main(args=None):
    rclpy.init(args=args)
    node = YoloRealSenseNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        node.get_logger().info("ノードを終了します")
    finally:
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()
