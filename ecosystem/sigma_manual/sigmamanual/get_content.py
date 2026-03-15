"""
Auto-split from ecosystem\sigma_manual.py — SigmaManual.get_content
"""

from typing import Dict, List, Any



class SigmaManual:
    def get_content(self, section: str) -> Dict[str, str]:
        return self.MANUAL_DATA.get(section, {})
