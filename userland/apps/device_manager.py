"""
device_manager.py — backward-compat shim.
Real implementation lives in device_manager/ package.
"""

from device_manager.SovereignDeviceManager import *  # noqa

__all__ = ['SovereignDeviceManager']
