"""
lidar関連nodeを立ち上げるlaunch
"""

import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription, GroupAction, TimerAction
from launch.substitutions import LaunchConfiguration, PathJoinSubstitution
from launch_ros.substitutions import FindPackageShare
from launch_ros.actions import Node, PushRosNamespace
from launch.launch_description_sources import PythonLaunchDescriptionSource


def generate_launch_description():
    robot_package_name = "tourobo_2026_foot"
    ld = LaunchDescription()
    package_dir = get_package_share_directory(robot_package_name)

    lidar_launch = IncludeLaunchDescription(
            PythonLaunchDescriptionSource(
                os.path.join(get_package_share_directory('urg_node2'), 'launch', 'urg_node2.launch.py')
            )
    )

    ld.add_action(lidar_launch)

    return ld
