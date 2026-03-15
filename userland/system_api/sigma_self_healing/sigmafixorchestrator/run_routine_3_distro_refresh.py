"""
Auto-split from userland\system_api\sigma_self_healing.py — SigmaFixOrchestrator.run_routine_3_distro_refresh
"""

import os
import sys
import json
import shutil
import importlib
import subprocess
import datetime
from typing import List, Dict, Any, Optional



class SigmaFixOrchestrator:
    def run_routine_3_distro_refresh(self):
        """Re-assembles the Distro IMG if files are corrupted."""
        self.log('Routine 3: Refreshing Sovereign Distro Image...')
        dist_img = os.path.join(self.root, 'SOVEREIGN_DISTRO_IMG')
        if os.path.exists(dist_img):
            shutil.copy2(os.path.join(self.root, 'SET_AS_NATIVE_BOOT.bat'), dist_img)
            self.log('Native Boot Scripts Refreshed.')
