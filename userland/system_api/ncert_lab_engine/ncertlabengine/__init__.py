# Generated method: NCERTLabEngine.__init__
import sys
import os
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class NCERTLabEngine:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self._phy = None
        self._chem = None
        self._bio = None
        self._math = None