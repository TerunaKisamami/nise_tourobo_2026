"""
action nodeを立ち上げるlaunch
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
    robot_package_name = "tourobo_2026_auto_base"
    ld = LaunchDescription()
    pkg_dir = get_package_share_directory(robot_package_name)

    map_pure_pursuit_config_path = os.path.join(pkg_dir, "config",
                                                "omni_pure_pursuit_map.yaml")
    odom_pure_pursuit_config_path = os.path.join(pkg_dir, "config",
                                                 "omni_pure_pursuit_odom.yaml")

    # ---- action node ----
    pure_pursuit_map_node = Node(
        package=robot_package_name,
        executable="omni_pure_pursuit_action_node_v2",
        name="pure_pursuit_map",
        parameters=[map_pure_pursuit_config_path],
    )

    pure_pursuit_odom_node = Node(
        package=robot_package_name,
        executable="omni_pure_pursuit_action_node_v2",
        name="pure_pursuit_odom",
        parameters=[odom_pure_pursuit_config_path],
    )

    ld.add_action(pure_pursuit_map_node)
    ld.add_action(pure_pursuit_odom_node)
