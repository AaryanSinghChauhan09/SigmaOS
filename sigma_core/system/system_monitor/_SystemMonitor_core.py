# Generated class core: SystemMonitor
import os
import sys
import time
from userland.system_api.sigma_std import SigmaSys
from sigma_core.hal.kernel_hal import SovereignHAL

class SystemMonitor:
    hal = SovereignHAL()
    ECO_THROTTLE_MS = 15000
    STATIC_THROTTLE_MS = 5000