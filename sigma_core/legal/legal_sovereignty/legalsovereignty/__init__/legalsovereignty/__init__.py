# Generated method: LegalSovereignty.__init__
import time
from typing import Dict, Any, List, Optional
from sigma_core.system.interfaces import SigmaModuleBase

class LegalSovereignty:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.kernel = kernel
        self.codes = {'BNSS': 'Bharatiya Nagarik Suraksha Sanhita (2023) - Procedural', 'BNS': 'Bharatiya Nyaya Sanhita (2023) - Substantive', 'BSA': 'Bharatiya Sakshya Adhiniyam (2023) - Evidence'}
        self.deadlines = {'charge_sheet_minor': 60.0, 'charge_sheet_major': 90.0, 'judgment_after_args': 30.0, 'appeal_limitation': 30.0}