# Generated method: SigmaAutomationLayer.__init__
import time
import json
import uuid
import threading
from pathlib import Path
from typing import Dict, List, Any

class SigmaAutomationLayer:
    def __init__(self, kernel):
        self.kernel = kernel
        self.config_dir = Path('C:/Users/SigmaUser\\.gemini\\antigravity\\scratch\\SigmaOS\\config\\automation')
        self.config_dir.mkdir(parents=True, exist_ok=True)
        self.backups_file = self.config_dir / 'scheduled_backups.json'
        self.users_file = self.config_dir / 'users.json'
        self.updates_file = self.config_dir / 'updates.json'
        self.backups_schedule = self._load_data(self.backups_file, [])
        self.users = self._load_data(self.users_file, {'root': {'uid': 0, 'groups': ['root', 'wheel', 'sudo']}})
        self.update_policy = self._load_data(self.updates_file, {'auto_update': True, 'channel': 'stable', 'time': '03:00'})
        self._start_automation_daemon()