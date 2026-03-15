"""
device_manager.py — backward-compat shim.
Real implementation lives in device_manager/ package.
"""

from .device_manager.SovereignDeviceManager import *  # noqa

__all__ = ['SovereignDeviceManager']

"""Auto-generated package __init__.py"""
from .sovereigndevicemanager import *  # noqa: F401, F403
