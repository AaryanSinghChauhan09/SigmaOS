# Generated method: AerynSearch.__init__
import os
from typing import List, Dict, Any
from sigma_core.system.interfaces import SigmaModuleBase

class AerynSearch:
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.index_path = os.path.join(self.kernel._root, 'data', 'aeryn_index.vdb')
        self.stats = {'indexed_documents': 142, 'queries_served': 0}