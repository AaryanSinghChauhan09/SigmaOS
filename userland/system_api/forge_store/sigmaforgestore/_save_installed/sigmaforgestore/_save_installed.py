# Generated method: SigmaForgeStore._save_installed
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
    def _save_installed(self):
        self._installed_apps_file.parent.mkdir(parents=True, exist_ok=True)
        with open(self._installed_apps_file, 'w', encoding='utf-8') as f:
            json.dump(self._installed_apps, f)