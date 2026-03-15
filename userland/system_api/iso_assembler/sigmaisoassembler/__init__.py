# Generated method: SigmaISOAssembler.__init__
import os
import json

class SigmaISOAssembler:
    def __init__(self, target_dir):
        self.target_dir = target_dir
        self.iso_root = os.path.join(self.target_dir, 'SOVEREIGN_DISTRO_IMG')