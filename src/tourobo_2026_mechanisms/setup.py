import os
from glob import glob
from setuptools import find_packages, setup

package_name = 'tourobo_2026_mechanisms'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
        (os.path.join('share', package_name, 'launch'), glob(os.path.join('launch', '*launch.[pxy][yma]*'))),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='hatsu',
    maintainer_email='teruna1493kisamami@gmail.com',
    description='TODO: Package description',
    license='TODO: License declaration',
    extras_require={
        'test': [
            'pytest',
        ],
    },
    entry_points={
        'console_scripts': [
            "yolo_node = tourobo_2026_mechanisms.yolo_node:main",
            "ball_get_node = tourobo_2026_mechanisms.ball_get_node:main",
            "ball_put_plate_node = tourobo_2026_mechanisms.ball_put_plate_node:main",
            "ball_put_gate_node = tourobo_2026_mechanisms.ball_put_gate_node:main",
            "ball_shoot_node = tourobo_2026_mechanisms.ball_shoot_node:main",
            "joy_mechanism_client = tourobo_2026_mechanisms.joy_mechanism_client:main",
            "ball_arm_operation_node = tourobo_2026_mechanisms.ball_arm_operation_node:main",
            "ball_intake_node = tourobo_2026_mechanisms.ball_intake_node:main",
            "ball_shoot_aim_node = tourobo_2026_mechanisms.ball_shoot_aim_node:main",
            "mechanism_reset_node = tourobo_2026_mechanisms.mechanism_reset_node:main",
            "ball_circle_node = tourobo_2026_mechanisms.ball_circle_node:main",
        ],
    },
)
