# Generated method: SigmaUserSupremacy.__init__
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaUserSupremacy:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.telemetry_killswitch = True
        self.forced_updates = False
        self.root_authority = 'USER_ONLY'