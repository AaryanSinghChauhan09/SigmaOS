# Generated method: SovereignAnalytics._ensure_log
import time
import psutil
import json
import os

class SovereignAnalytics:
    def _ensure_log(self):
        if not os.path.exists(self.log_path):
            with open(self.log_path, 'w') as f:
                json.dump({'sessions': [], 'metrics': {}}, f)