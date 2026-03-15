# Generated method: NCERTLabEngine._load_shards
import sys
import os
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class NCERTLabEngine:
    def _load_shards(self):
        """USP: Modular Shard Injection. Loads subject-specific labs dynamically."""
        try:
            from userland.apps.ncert_physics_lab import Physics_Classes_11_12
            from userland.apps.ncert_chemistry_lab import Chemistry_Classes_11_12
            from userland.apps.ncert_biology_lab import Biology_Classes_11_12
            from userland.apps.ncert_maths_lab import Maths_Classes_11_12
            self._phy = Physics_Classes_11_12
            self._chem = Chemistry_Classes_11_12
            self._bio = Biology_Classes_11_12
            self._math = Maths_Classes_11_12
        except Exception as e:
            print(f'[NCERT] Shard load error: {str(e)}')