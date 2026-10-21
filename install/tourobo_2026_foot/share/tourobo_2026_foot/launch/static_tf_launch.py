"""
静的tfを立ち上げるlaunch
base_link->lidar
base_link->footprint
base_link->imu_link
"""

import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription, GroupAction, TimerAction
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node, PushRosNamespace
from launch.launch_description_sources import PythonLaunchDescriptionSource


def generate_launch_description():
    robot_package_name = "tourobo_2026_foot"
    ld = LaunchDescription()
    pkg_dir = get_package_share_directory(robot_package_name)

    # ---- 静的tfの配信----
    static_tf_hokuyo = Node(
        package="tf2_ros",
        executable="static_transform_publisher",
        output="screen",
        arguments=[
            "-0.310",        #x
            "0.0",           #y    
            "0.0",           #z
            "3.1416",        #roll
            "0.0",           #pitch
            "0.0",           #yaw
            "base_link",
            "laser",
        ],
    )

    # base_link -> base_footprint
    static_transform_publisher_footprint_node = Node(
        package="tf2_ros",
        executable="static_transform_publisher",
        output="screen",
        arguments=[
            "0.0",
            "0.0",
            "0.0",
            "0.0",
            "0.0",
            "0.0",
            "base_link",
            "base_footprint",
        ],
    )

    # base_link->imu_link
    static_transform_publisher_imu_node = Node(
        package="tf2_ros",
        executable="static_transform_publisher",
        arguments=[
            "-0.310",
            "0.0",
            "0.0",
            "0.0",
            "0.0",
            "0.0",
            "base_link",
            "imu_link",
        ])

    ld.add_action(static_tf_hokuyo)
    ld.add_action(static_transform_publisher_imu_node)
    ld.add_action(static_transform_publisher_footprint_node)

    return ld
