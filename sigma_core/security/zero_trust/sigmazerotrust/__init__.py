# Generated method: SigmaZeroTrust.__init__
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaZeroTrust:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.trust_levels = {}