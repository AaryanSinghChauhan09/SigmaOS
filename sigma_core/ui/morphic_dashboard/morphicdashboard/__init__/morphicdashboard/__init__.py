# Generated method: MorphicDashboard.__init__
import os
import sys
import time
import random
from sigma_core.system.interfaces import SigmaModuleBase

class MorphicDashboard:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.active_view = 'SYS_HEALTH'
        self.refresh_rate = 1.0
        self.telemetry_history = []