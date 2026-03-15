# Generated method: SovereignApp.__init__
from sigma_core.system.interfaces import SigmaModuleBase

class SovereignApp:
    def __init__(self, kernel=None, *args, **kwargs):
        super().__init__(kernel)
        self.app_id = self.__class__.__name__