# Generated method: MinimalistController.__init__
from typing import List, Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class MinimalistController:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.kernel = kernel
        self.active_mode = 'STANDARD'
        self.non_essential_shards = ['gamification', 'nexus', 'portal', 'vision']