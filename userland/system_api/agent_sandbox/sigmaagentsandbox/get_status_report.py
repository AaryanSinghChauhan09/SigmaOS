# Generated method: SigmaAgentSandbox.get_status_report
import os
import shutil
import uuid
import time
import subprocess
import threading
from typing import Dict, Any, List

class SigmaAgentSandbox:
    def get_status_report(self) -> List[Dict]:
        return [{'id': k, 'name': v['name'], 'status': v['status'], 'violations': len(v['violations'])} for k, v in self._active_silos.items()]