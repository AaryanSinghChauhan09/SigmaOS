# Generated method: SigmaForgeStore.get_catalog
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
    def get_catalog(self, category: str=None) -> Dict[str, Any]:
        """Return full catalog or filter by category."""
        if category:
            return {k: v for k, v in self.catalog.items() if v.get('category', '').lower() == category.lower()}
        return self.catalog