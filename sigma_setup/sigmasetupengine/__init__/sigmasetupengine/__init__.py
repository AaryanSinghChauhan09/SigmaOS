# Generated method: SigmaSetupEngine.__init__
import os
import sys
import platform
import shutil
import time
import subprocess
from pathlib import Path

class SigmaSetupEngine:
    def __init__(self):
        self.root = Path(os.path.abspath(os.path.dirname(__file__)))
        self.version = '1.0.0-PRO-SETUP'
        self.prereqs = ['python', 'pip', 'git', 'powershell']