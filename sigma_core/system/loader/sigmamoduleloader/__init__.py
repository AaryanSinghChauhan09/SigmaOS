# Generated method: SigmaModuleLoader.__init__
import importlib
from .interfaces import ISigmaModule, ISigmaService

class SigmaModuleLoader:
    def __init__(self, kernel):
        self.kernel = kernel