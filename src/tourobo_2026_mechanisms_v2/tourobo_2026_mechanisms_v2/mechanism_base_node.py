import rclpy
from rclpy.node import Node
from dyna_interfaces.msg import DynaTarget


class Params:
    def __init__(self, node):
        self.node = node
    def __getattr__(self, name):
        return self.node.get_parameter(name).value

class MechanismBaseNode(Node):
    def __init__(self, node_name):
        super().__init__(node_name)
        self.declare_parameters(
            namespace='',
            parameters=[
                ('left_roller_can_id', 0x010),
                ('right_roller_can_id', 0x041),
                ('down_roller_can_id', 0x040),
                ('shoot_roller_1_can_id', 0x011),
                ('shoot_roller_2_can_id', 0x012),
                ('shoot_roller_3_can_id', 0x013),
                ('mini_shoot_can_id', 0x031),
                ('arm_left_id', 0),
                ('arm_right_id', 1),
                ('guard_left_id', 2),
                ('guard_right_id', 3),
                ('shoot_angle_id', 4),
                ('arm_left_open', 2000),
                ('arm_right_open', 2000),
                ('arm_left_close', 0),
                ('arm_right_close', 0),
                ('arm_left_get_half', 1000),
                ('arm_right_get_half', 1000),
                ('guard_left_open', 2000),
                ('guard_right_open', 2000),
                ('guard_left_close', 0),
                ('guard_right_close', 0),
                ('shoot_angle_min', 0),
                ('shoot_angle_max', 2000),
                ('shoot_angle_at_gate', 1000),
                ('shoot_push_max', 2000),
                ('shoot_push_min', 0),
                ('shoot_push_intake_gate_ready', 100),
                ('shoot_push_intake_shoot_ready', 200),
                ('shoot_push_shoot_finish', 300),
                ('shoot_push_put_gate_finish', 400),
                ('shoot_motor_speed', 1000),
                ('ball_get_down_roller_speed', 1000),
                ('ball_get_up_roller_speed', -1000),
                ('ball_intake_down_roller_speed', 1000),
                ('ball_intake_up_roller_speed', -1000),
                ('ball_put_plate_down_roller_speed', 1000),
                ('ball_put_plate_up_roller_speed', -1000),
                ('wait_time_guard', 1.0),
                ('wait_time_arm', 1.0),
                ('wait_time_push', 1.0),
                ('wait_time_intake', 1.0),
                ('wait_time_put_plate', 1.0),
                ('wait_time_get', 1.0),
                ('wait_time_shoot_dir', 1.5)
            ]
        )

        self.dyna_extpos_publisher = self.create_publisher(DynaTarget, "/dyna_target_extpos", 10)
        self.dyna_vel_publisher = self.create_publisher(DynaTarget, "/dyna_target_vel", 10)
        self.dyna_pos_publisher = self.create_publisher(DynaTarget, "/dyna_target_pos", 10)

        self.p = Params(self)

    def get_p(self, name):
        return self.get_parameter(name).value

    def publish_dyna_pos(self, motor_id, target):
        msg = DynaTarget()
        msg.id = motor_id
        msg.target = target
        self.dyna_pos_publisher.publish(msg)

    def publish_dyna_extpos(self, motor_id, target):
        msg = DynaTarget()
        msg.id = motor_id
        msg.target = target
        self.dyna_extpos_publisher.publish(msg)

