"""
メカニズム関連のnodeを立ち上げるlaunchですわよ
"""
import launch
from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    pkg_name = 'tourobo_2026_mechanisms'
    
    ball_get = Node(package=pkg_name, executable="ball_get_node", name="ball_get_node")
    ball_put_plate = Node(package=pkg_name, executable="ball_put_plate_node", name="ball_put_plate_node")
    ball_put_gate = Node(package=pkg_name, executable="ball_put_gate_node", name="ball_put_gate_node")
    ball_shoot = Node(package=pkg_name, executable="ball_shoot_node", name="ball_shoot_node")
    ball_gate_operation = Node(package=pkg_name, executable="ball_gate_operation_node", name="ball_gate_operation_node")
    ball_intake = Node(package=pkg_name, executable="ball_intake_node", name="ball_intake_node")
    ball_shoot_aim = Node(package=pkg_name, executable="ball_shoot_aim_node", name="ball_shoot_aim_node")
    
    joy_client = Node(package=pkg_name, executable="joy_mechanism_client", name="joy_mechanism_client")

    ld = LaunchDescription()

    ld.add_action(ball_get)
    ld.add_action(ball_put_plate)
    ld.add_action(ball_put_gate)
    ld.add_action(ball_shoot)
    ld.add_action(ball_gate_operation)
    ld.add_action(ball_intake)
    ld.add_action(ball_shoot_aim)
    ld.add_action(joy_client)

    return ld
