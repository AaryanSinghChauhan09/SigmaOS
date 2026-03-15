# Generated method: ZeroTrustValidator.__init__
import hashlib
import json
import re
from sigma_core.system.interfaces import ISigmaModule, SigmaModuleBase

class ZeroTrustValidator:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self._trusted_keys = ['cosmos_root_v1', 'antigravity_core_v1']