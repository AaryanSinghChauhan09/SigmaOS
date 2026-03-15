# Generated method: SigmaSys.sensors_battery
import sys
import os
import time
import json
import hashlib
import hmac
import urllib.request
import subprocess

class SigmaSys:
    @staticmethod
    def sensors_battery():

        class BatteryStatus:

            def __init__(self, percent, secsleft, power_plugged):
                self.percent = percent
                self.secsleft = secsleft
                self.power_plugged = power_plugged
        if sys.platform == 'win32':
            try:
                import ctypes
                from ctypes import wintypes

                class SYSTEM_POWER_STATUS(ctypes.Structure):
                    _fields_ = [('ACLineStatus', wintypes.BYTE), ('BatteryFlag', wintypes.BYTE), ('BatteryLifePercent', wintypes.BYTE), ('SystemStatusFlag', wintypes.BYTE), ('BatteryLifeTime', wintypes.DWORD), ('BatteryFullLifeTime', wintypes.DWORD)]
                power_status = SYSTEM_POWER_STATUS()
                if ctypes.windll.kernel32.GetSystemPowerStatus(ctypes.byref(power_status)):
                    percent = power_status.BatteryLifePercent
                    plugged = bool(power_status.ACLineStatus == 1)
                    secsleft = power_status.BatteryLifeTime
                    if percent == 255:
                        return None
                    if secsleft == 4294967295 or plugged:
                        secsleft = -2 if plugged else -1
                    return BatteryStatus(float(percent), int(secsleft), plugged)
            except Exception:
                pass
        return None