"""
Auto-split from userland\system_api\linux_parity_engine.py — LinuxParityGapAnalysis.get_critical_gaps
"""

import time
import uuid
import random
from typing import Dict, List, Any



class LinuxParityGapAnalysis:
    def get_critical_gaps(self) -> List[str]:
        """Returns only the unimplemented items needing urgent attention."""
        gaps = []
        for feat, status in self.SIGMA_STATUS.items():
            if 'GAP' in status or 'PLANNED' in status:
                gaps.append(f'  ⚠  {feat}: {status}')
        return gaps
