# Generated method: SigmaSnapshotEngine.create_snapshot
import time
import uuid
import random
from typing import Dict, List, Any

class SigmaSnapshotEngine:
    def create_snapshot(self, label: str='manual', snap_type: str='manual') -> Dict:
        snap_id = f'snap-{uuid.uuid4().hex[:6].upper()}'
        size = f'{round(random.uniform(4.0, 8.5), 1)}GB'
        snap = {'id': snap_id, 'date': time.strftime('%Y-%m-%d %H:%M'), 'label': label, 'size': size, 'type': snap_type}
        self._snapshots.append(snap)
        return {'status': 'OK', 'message': f"[snapshot] Created '{snap_id}': {label} ({size}).", 'snap': snap}