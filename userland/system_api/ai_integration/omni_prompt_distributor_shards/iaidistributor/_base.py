import time
import os
from abc import ABC, abstractmethod
from sigma_core.interfaces.base_sovereign import SovereignModule


class IAIDistributor(ABC):

    def distribute(self, *args, **kwargs):
        import importlib
        mod = importlib.import_module('.distribute', package=__package__)
        return getattr(mod, 'distribute')(self, *args, **kwargs)