# Generated method: SigmaNomad.__init__
import os
import sys
import subprocess
import json

class SigmaNomad:
    def __init__(self, mode='Virtual'):
        self.mode = mode
        self.root_dir = os.path.abspath(os.path.dirname(__file__))
        self.portable_drive = os.path.join(self.root_dir, 'sigma_portable_drive')
        if not os.path.exists(self.portable_drive):
            os.makedirs(self.portable_drive)
            os.makedirs(os.path.join(self.portable_drive, 'home'))
            os.makedirs(os.path.join(self.portable_drive, 'mnt/shared'))