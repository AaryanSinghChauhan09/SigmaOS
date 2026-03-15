# Generated method: UniversalBridge.execute_app
import sys
import os
import subprocess
from typing import Dict, Any, Optional
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class UniversalBridge:
    def execute_app(self, path: str, context: Optional[str]=None) -> bool:
        """USP: Intelligent Binary Dispatching. Detects format and spawns correct bridge."""
        if path.endswith('.exe') or path.endswith('.msi'):
            return self._spawn_win_bridge(path)
        elif path.endswith('.apk'):
            return self._spawn_android_bridge(path)
        elif os.access(path, os.X_OK):
            return self._spawn_linux_bridge(path)
        return False