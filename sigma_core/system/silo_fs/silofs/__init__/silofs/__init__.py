# Generated method: SiloFS.__init__
import os
import shutil
from typing import List, Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SiloFS:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.active_silos = {}
        self.sandbox_root = os.path.join(self.kernel._root, 'data', 'sandboxes')
        if not os.path.exists(self.sandbox_root):
            os.makedirs(self.sandbox_root)