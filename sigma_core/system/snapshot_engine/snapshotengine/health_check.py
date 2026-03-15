# Generated method: SnapshotEngine.health_check
import os
import time
import json
from typing import Dict, Any, List
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SnapshotEngine:
    def health_check(self) -> str:
        return f"OK — Snapshots: {self.stats['snapshots_captured']} | Last Capture: {self.stats['avg_capture_ms']:.2f}ms"