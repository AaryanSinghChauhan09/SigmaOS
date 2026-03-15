# Generated method: SigmaFS.get_volume_stats
import time
import hashlib
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from pathlib import PurePosixPath

class SigmaFS:
    def get_volume_stats(self) -> dict:
        total_bytes = sum((n.size_bytes for n in self._inodes.values()))
        return {'fs': self.FS_VERSION, 'label': self.volume_label, 'inodes': len(self._inodes), 'snapshots': len(self._snapshots), 'cache_efficiency': f"{self._stats['cache_hits'] / max(self._stats['reads'], 1):.1%}", 'total_data_kb': int(float(total_bytes) / 10.24) / 100.0, 'stats': self._stats}