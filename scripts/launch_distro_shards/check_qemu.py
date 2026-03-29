"""
SigmaOS Apex Shard: check_qemu
"""
import os
import subprocess
import sys


def check_qemu():
    try:
        subprocess.run(['qemu-system-x86_64', '--version'], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        return True
    except FileNotFoundError:
        return False