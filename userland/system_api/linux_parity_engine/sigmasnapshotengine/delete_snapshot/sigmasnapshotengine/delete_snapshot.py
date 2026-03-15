# Generated method: SigmaSnapshotEngine.delete_snapshot
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaSnapshotEngine:
    def delete_snapshot(self, snap_id: str) -> str:
        before = len(self._snapshots)
        self._snapshots = [s for s in self._snapshots if s['id'] != snap_id]
        if len(self._snapshots) < before:
            return f"[snapshot] Deleted '{snap_id}'."
        return f"[snapshot] Snapshot '{snap_id}' not found."