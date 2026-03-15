# Generated method: ApexPerformanceSuite.run_all
import sys
import os
import time
import json
import statistics
import threading
from sigma_core.kernel import SigmaKernel
from sigma_core.system.sigma_fs import SigmaFS, BlockHealth
from sigma_core.ui.fluid_design import FluidTheme, THEMES

class ApexPerformanceSuite:
    def run_all(self):
        print('🚀 STARTING SIGMA OS APEX PERFORMANCE SUITE...')
        self.test_kernel_boot()
        self.test_fs_performance()
        self.test_governor_vibe_latency()
        self.test_app_hydration()
        self.report()