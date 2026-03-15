# Generated method: SigmaFixOrchestrator.run_full_audit
import os
import sys
import json
import shutil
import importlib
import subprocess
import datetime
from typing import List, Dict, Any, Optional

class SigmaFixOrchestrator:
    def run_full_audit(self) -> None:
        """Executes all fix routines in sequence."""
        self.log('--- STARTING FULL KERNEL AUDIT & REPAIR ---')
        self.run_routine_1_display_reset()
        self.run_routine_2_shell_recovery()
        self.run_routine_3_distro_refresh()
        self.run_routine_4_zram_purge()
        self.run_routine_5_io_accelerator()
        self.run_routine_6_privacy_hardener()
        self.run_routine_7_dependency_resolver()
        self.run_routine_8_config_vault_audit()
        self.run_routine_9_autonomous_kernel_patching()
        self.log('--- SYSTEM PURIFIED & FULLY FUNCTIONAL ---')