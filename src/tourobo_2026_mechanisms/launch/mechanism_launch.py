"""
メカニズム関連のnodeを立ち上げるlaunchですわよ
"""
from nise_tourobo_2026.src.tourobo_2026_mechanisms.launch.mechanism_launch import WAIT_TIME_ROLLER
import launch
from launch import LaunchDescription
from launch_ros.actions import Node

#各モーターCANのID
#下ローラー
DOWN_ROLLER_CAN_ID = 0x040
#右ローラー
RIGHT_ROLLER_CAN_ID = 0x041
#左ローラー
LEFT_ROLLER_CAN_ID = 0x010
#射出機構ローラー
SHOOT_ROLLER_1_CAN_ID = 0x011
SHOOT_ROLLER_2_CAN_ID = 0x012
SHOOT_ROLLER_3_CAN_ID = 0x013
#小ロボマス(射出機構用)
MINI_SHOOT_CAN_ID = 0x031

#ダイナミクセルID(仮置き)
ARM_LEFT_ID = 20
ARM_RIGHT_ID = 21
GUARD_LEFT_ID = 22
GUARD_RIGHT_ID = 23
SHOOT_ANGLE_ID = 10

#左右アーム開閉時のダイナミクセル値
ARM_OPEN = 2000 
ARM_CLOSE= 0
ARM_GET_HALF = 1000

#ガード開閉時のダイナミクセル値
GUARD_OPEN = 2000 
GUARD_CLOSE = 0

#射出機構の角度の最小値と最大値
SHOOT_ANGLE_MIN = 0
SHOOT_ANGLE_MAX = 2000
#城門位置の射出機構角度
SHOOT_ANGLE_AT_GATE = 1000 

#射出機構押し出し機構の位置(初期位置からの相対角度)
SHOOT_PUSH_MIN = 0 #これを初期位置にする
SHOOT_PUSH_MAX = 2000
SHOOT_PUSH_INTAKE_GATE_READY = 100
SHOOT_PUSH_INTAKE_SHOOT_READY = 200
SHOOT_PUSH_SHOOT_FINISH = 300
SHOOT_PUSH_PUT_GATE_FINISH = 400

#射出機構のローラー回転速度
SHOOT_MOTOR_SPEED = 1000

#ボール入手のローラー回転速度
BALL_GET_DOWN_ROLLER_SPEED = 1000
BALL_GET_UP_ROLLER_SPEED = -1000

#ボール内側取り込みローラー回転速度
BALL_INTAKE_DOWN_ROLLER_SPEED = 1000
BALL_INTAKE_UP_ROLLER_SPEED = -1000

#ぼーるを関所におくときのローラー回転速度
BALL_PUT_PLATE_DOWN_ROLLER_SPEED = 1000
BALL_PUT_PLATE_UP_ROLLER_SPEED = -1000

# 動作待機時間の設定
# ガードの開閉にかかる時間
WAIT_TIME_GUARD = 1.0
# アームの開閉にかかる時間
WAIT_TIME_ARM = 1.0
# ボール入手の時にボールが入るのを待つ時間
WAIT_TIME_GET = 1.0
# ボール内側取り込み時にボールが内部に入るのを待つ時間
WAIT_TIME_INTAKE = 1.0
#射出機構の角度変更にかかる時間
WAIT_TIME_SHOOT_DIR = 1.5
# ボールを関所に置くときに内部を移動するのを待つ時間
WAIT_TIME_PUT_PLATE = 1.0
# 射出機構の押し出し部分の動作時間
WAIT_TIME_PUSH = 1.0

def generate_launch_description():
    pkg_name = 'tourobo_2026_mechanisms'
    
    params = {
        'down_roller_can_id': DOWN_ROLLER_CAN_ID,
        'right_roller_can_id': RIGHT_ROLLER_CAN_ID,
        'left_roller_can_id': LEFT_ROLLER_CAN_ID,
        'shoot_roller_1_can_id': SHOOT_ROLLER_1_CAN_ID,
        'shoot_roller_2_can_id': SHOOT_ROLLER_2_CAN_ID,
        'shoot_roller_3_can_id': SHOOT_ROLLER_3_CAN_ID,
        'mini_shoot_can_id': MINI_SHOOT_CAN_ID,
        'arm_left_id': ARM_LEFT_ID,
        'arm_right_id': ARM_RIGHT_ID,
        'guard_left_id': GUARD_LEFT_ID,
        'guard_right_id': GUARD_RIGHT_ID,
        'shoot_angle_id': SHOOT_ANGLE_ID,
        'arm_open': ARM_OPEN,
        'arm_close': ARM_CLOSE,
        'guard_open': GUARD_OPEN,
        'guard_close': GUARD_CLOSE,
        'shoot_angle_min': SHOOT_ANGLE_MIN,
        'shoot_angle_max': SHOOT_ANGLE_MAX,
        'shoot_angle_at_gate': SHOOT_ANGLE_AT_GATE,
        'shoot_push_max': SHOOT_PUSH_MAX,
        'shoot_push_min': SHOOT_PUSH_MIN,
        'shoot_push_intake_gate_ready': SHOOT_PUSH_INTAKE_GATE_READY,
        'shoot_push_intake_shoot_ready': SHOOT_PUSH_INTAKE_SHOOT_READY,
        'shoot_push_shoot_finish': SHOOT_PUSH_SHOOT_FINISH,
        'shoot_push_put_gate_finish': SHOOT_PUSH_PUT_GATE_FINISH,
        'shoot_motor_speed': SHOOT_MOTOR_SPEED,
        'ball_get_down_roller_speed': BALL_GET_DOWN_ROLLER_SPEED,
        'ball_get_up_roller_speed': BALL_GET_UP_ROLLER_SPEED,
        'ball_intake_down_roller_speed': BALL_INTAKE_DOWN_ROLLER_SPEED,
        'ball_intake_up_roller_speed': BALL_INTAKE_UP_ROLLER_SPEED,
        'ball_put_plate_down_roller_speed': BALL_PUT_PLATE_DOWN_ROLLER_SPEED,
        'ball_put_plate_up_roller_speed': BALL_PUT_PLATE_UP_ROLLER_SPEED,
        'wait_time_guard': WAIT_TIME_GUARD,
        'wait_time_arm': WAIT_TIME_ARM,
        'wait_time_push': WAIT_TIME_PUSH,
        'wait_time_intake': WAIT_TIME_INTAKE,
        'wait_time_put_plate': WAIT_TIME_PUT_PLATE,
        'wait_time_get': WAIT_TIME_GET,
        'wait_time_shoot_dir': WAIT_TIME_SHOOT_DIR,
        'arm_get_half': ARM_GET_HALF,
    }
    
    ball_get = Node(package=pkg_name, executable="ball_get_node", name="ball_get_node", parameters=[params])
    ball_put_plate = Node(package=pkg_name, executable="ball_put_plate_node", name="ball_put_plate_node", parameters=[params])
    ball_put_gate = Node(package=pkg_name, executable="ball_put_gate_node", name="ball_put_gate_node", parameters=[params])
    ball_shoot = Node(package=pkg_name, executable="ball_shoot_node", name="ball_shoot_node", parameters=[params])
    ball_gate_operation = Node(package=pkg_name, executable="ball_gate_operation_node", name="ball_gate_operation_node", parameters=[params])
    ball_intake = Node(package=pkg_name, executable="ball_intake_node", name="ball_intake_node", parameters=[params])
    ball_shoot_aim = Node(package=pkg_name, executable="ball_shoot_aim_node", name="ball_shoot_aim_node", parameters=[params])
    
    joy_client = Node(package=pkg_name, executable="joy_mechanism_client", name="joy_mechanism_client", parameters=[params])

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
