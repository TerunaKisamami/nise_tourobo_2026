"""
メカニズム関連のnodeを立ち上げるlaunchですわよ
"""
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.actions import IncludeLaunchDescription
import os
from ament_index_python.packages import get_package_share_directory
import launch
from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    pkg_name = 'tourobo_2026_mechanisms'
    
#Nodes
    ball_get = Node(package=pkg_name, executable="ball_get_node", name="ball_get_node")
    ball_put_plate = Node(package=pkg_name, executable="ball_put_plate_node", name="ball_put_plate_node")
    ball_put_gate = Node(package=pkg_name, executable="ball_put_gate_node", name="ball_put_gate_node")
    ball_shoot = Node(package=pkg_name, executable="ball_shoot_node", name="ball_shoot_node")
    ball_arm_operation = Node(package=pkg_name, executable="ball_arm_operation_node", name="ball_arm_operation_node")
    ball_intake = Node(package=pkg_name, executable="ball_intake_node", name="ball_intake_node")
    ball_shoot_aim = Node(package=pkg_name, executable="ball_shoot_aim_node", name="ball_shoot_aim_node")
    ball_vomit_plate = Node(package=pkg_name, executable="ball_vomit_plate_node", name="ball_vomit_plate_node")

    joy_client = Node(package=pkg_name, executable="joy_mechanism_client", name="joy_mechanism_client")

    ld = LaunchDescription()

    ld.add_action(ball_get)
    ld.add_action(ball_put_plate)
    ld.add_action(ball_put_gate)
    ld.add_action(ball_shoot)
    ld.add_action(ball_arm_operation)
    ld.add_action(ball_intake)
#    ld.add_action(ball_shoot_aim)
    ld.add_action(ball_vomit_plate)
    ld.add_action(joy_client)

    return ld
