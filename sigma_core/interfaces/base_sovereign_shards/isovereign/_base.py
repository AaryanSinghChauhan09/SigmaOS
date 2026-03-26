from abc import ABC, abstractmethod
import time


class ISovereign(ABC):

    def execute(self, *args, **kwargs):
        import importlib
        mod = importlib.import_module('.execute', package=__package__)
        return getattr(mod, 'execute')(self, *args, **kwargs)

    def health_check(self, *args, **kwargs):
        import importlib
        mod = importlib.import_module('.health_check', package=__package__)
        return getattr(mod, 'health_check')(self, *args, **kwargs)