# Generated method: SigmaSetupEngine.setup_package_manager
import os
import sys
import platform
import shutil
import time
import subprocess
from pathlib import Path

class SigmaSetupEngine:
    def setup_package_manager(self):
        """Initializes the Sovereign App Store (Package Registry)."""
        print('[*] Initializing Sovereign App Store Registry...')
        registry_path = self.root / 'ecosystem' / 'registry.json'
        if not registry_path.exists():
            import json
            initial_data = {'os_version': '2.0.0', 'installed_apps': ['image_to_text', 'text_to_html', 'video_transcript'], 'available_repo': 'https://sigmaos.sovereign/repo', 'last_sync': time.time()}
            with open(registry_path, 'w') as f:
                json.dump(initial_data, f, indent=4)
            print('[+] Registry created: ecosystem/registry.json')