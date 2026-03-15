# Generated method: SigmaModuleBase.__init__
import hashlib
import json
import re
from sigma_core.system.interfaces import ISigmaModule, SigmaModuleBase

class SigmaModuleBase:
    def __init__(self, kernel):
        self.kernel = kernel