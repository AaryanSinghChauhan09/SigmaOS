# Resilient Method: UniversalRegistry.__init__
from sigma_core.security.resilience_guard import resilient_module
import os
import importlib.util
import sys

class UniversalRegistry:
    @resilient_module
    def __init__(self, root_dir):
        self.root_dir = root_dir
        self.cache = {}