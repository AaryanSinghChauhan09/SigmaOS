# Generated method: SigmaForgeStore._load_installed
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
    def _load_installed(self) -> List[str]:
        if self._installed_apps_file.exists():
            try:
                with open(self._installed_apps_file, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except Exception:
                pass
        return []