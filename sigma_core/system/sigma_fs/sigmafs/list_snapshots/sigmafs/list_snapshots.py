# Generated method: SigmaFS.list_snapshots
import time
import hashlib
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from pathlib import PurePosixPath

class SigmaFS:
    def list_snapshots(self) -> list[dict]:
        return [{'snap_id': s.snap_id, 'label': s.label, 'timestamp': s.timestamp, 'size_kb': s.size_kb} for s in self._snapshots.values()]