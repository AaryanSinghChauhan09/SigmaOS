"""
Auto-split from userland\system_api\forge_store.py — SigmaForgeStore.install_app
"""

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
    def install_app(self, app_id: str) -> Dict[str, Any]:
        if app_id not in self.catalog:
            return {'status': 'ERROR', 'msg': 'App not found.'}
        if app_id in self._installed_apps:
            return {'status': 'OK', 'msg': 'Already installed.'}
        self._installed_apps.append(app_id)
        self._save_installed()
        return {'status': 'SUCCESS', 'msg': f"{self.catalog[app_id]['name']} installed successfully (Compressed)."}
