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

#ダイナミクセルID
LEFT_ARM_ID = 0
RIGHT_ARM_ID = 1
LEFT_GUARD_ID = 2
RIGHT_GUARD_ID = 3
SHOOT_ANGLE_ID = 4
SHOOT_DIRECTION_ID = 4 # Alias if any

#左右アーム開閉時のダイナミクセル値
LEFT_ARM_OPEN =  1300#内部的には2630
RIGHT_ARM_OPEN = -1176 #内部的には1040
LEFT_ARM_CLOSE = 0 #内部的には1467
RIGHT_ARM_CLOSE = 0 #内部的には2216
LEFT_ARM_GET_HALF = 784 #内部的には2251
RIGHT_ARM_GET_HALF = -863 #内部的には1353

#ガード開閉時のダイナミクセル値
LEFT_GUARD_OPEN = 0 #内部的には317
RIGHT_GUARD_OPEN = 0 #内部的には1502
LEFT_GUARD_CLOSE = 1156 #内部的には1473
RIGHT_GUARD_CLOSE = -1204 #内部的には298

#射出機構の角度の最小値と最大値
SHOOT_ANGLE_MIN = 0 #内部的には3000
SHOOT_ANGLE_MAX =800  #内部的には3800
#城門位置の射出機構角度
SHOOT_ANGLE_AT_GATE = 695 #内部的には3695

#射出機構押し出し機構の位置(初期位置からの相対角度)
SHOOT_PUSH_MIN = 0 #これを初期位置にする
SHOOT_PUSH_MAX = 78000 #射出機構に近いほどでかい

#射出機構のローラー回転速度
SHOOT_MOTOR_SPEED = 800

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
WAIT_TIME_GET = 0.7
# ボール内側取り込み時にボールが内部に入るのを待つ時間
WAIT_TIME_INTAKE = 2.0
#射出機構の角度変更にかかる時間
WAIT_TIME_SHOOT_DIR = 1.5
WAIT_TIME_SHOOT_DIR_PUT_GATE = 1.5 # Alias if any
# ボールを関所に置くときに内部を移動するのを待つ時間
WAIT_TIME_PUT_PLATE = 1.0
# 射出機構の押し出し部分の動作時間
WAIT_TIME_PUSH =6.0
WAIT_TIME_ROLLER = 1.0

# 状態定数 (not currently used but to resolve missing dict keys if any)
SHOOT_PUSH_INTAKE_GATE_READY = 100
SHOOT_PUSH_INTAKE_SHOOT_READY = 200
SHOOT_PUSH_SHOOT_FINISH = 300
SHOOT_PUSH_PUT_GATE_FINISH = 400
