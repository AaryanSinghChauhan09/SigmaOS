# Generated method: SigmaFS._calc_dedup_ratio
import time
import hashlib
import uuid
import random
from dataclasses import dataclass, field
from enum import Enum, auto
from pathlib import PurePosixPath

class SigmaFS:
    def _calc_dedup_ratio(self) -> str:
        if not self._inodes:
            return 'N/A'
        ratio = 1.0 - len(self._dedup) / max(len(self._inodes), 1)
        return f'{ratio:.1%} dedup savings'