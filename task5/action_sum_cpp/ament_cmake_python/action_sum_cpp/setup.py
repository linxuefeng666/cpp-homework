from setuptools import find_packages
from setuptools import setup

setup(
    name='action_sum_cpp',
    version='0.0.0',
    packages=find_packages(
        include=('action_sum_cpp', 'action_sum_cpp.*')),
)
