from setuptools import find_packages, setup

package_name = 'tourobo_2026_auto_strategy'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
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
            'main_strategy_node = tourobo_2026_auto_strategy.main_strategy_node:main',
        ],
    },
)
