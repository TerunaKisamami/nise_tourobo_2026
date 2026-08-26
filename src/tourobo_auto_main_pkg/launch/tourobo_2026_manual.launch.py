import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import IncludeLaunchDescription
from launch.launch_description_sources import PythonLaunchDescriptionSource

def generate_launch_description():
    mechanism_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(
                get_package_share_directory('tourobo_2026_mechanisms'),
                'launch',
                'mechanism_launch.py'
            )
        )
    )

    foot_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            os.path.join(
                get_package_share_directory('tourobo_2026_foot'),
                'launch',
                'foot_launch.py'
            )
        )
    )

    ld = LaunchDescription()
    
    ld.add_action(mechanism_launch)
    ld.add_action(foot_launch)

    return ld
