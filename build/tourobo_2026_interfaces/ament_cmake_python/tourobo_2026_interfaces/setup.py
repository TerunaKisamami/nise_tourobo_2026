from setuptools import find_packages
from setuptools import setup

setup(
    name='tourobo_2026_interfaces',
    version='0.0.0',
    packages=find_packages(
        include=('tourobo_2026_interfaces', 'tourobo_2026_interfaces.*')),
)
