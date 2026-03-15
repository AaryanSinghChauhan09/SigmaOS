# Generated method: SigmaForgeStore.get_catalog_list
import json
import os
from pathlib import Path
from typing import Dict, List, Any
from conversion_engine import SigmaConversionEngine
from sovereign_utility_suite import SovereignUtilitySuite
from ad_shield import SigmaAdShield
from youtube_downloader import SigmaYouTubeSovereignFetcher
from sovereign_clipboard import SigmaSovereignClipboard
from agentic_claw import SigmaAgenticClaw
from sovereign_scheduler import SigmaSovereignScheduler
from sigma_gateway import SigmaGatewayAgent
from dev_liaison import SigmaDevLiaison
from sovereign_lab import SovereignLab
from sovereign_legal_academy import SovereignLegalAcademy

class SigmaForgeStore:
    def get_catalog_list(self, category: str=None) -> List[dict]:
        """Return catalog as a list suitable for the app store grid view."""
        filtered = self.get_catalog(category)
        result = []
        for app_id, meta in filtered.items():
            entry = dict(meta)
            entry['app_id'] = app_id
            entry['installed'] = app_id in self._installed_apps
            result.append(entry)
        return result