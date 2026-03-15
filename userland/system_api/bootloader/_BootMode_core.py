# Generated class core: BootMode
import time
import uuid
import hashlib
from dataclasses import dataclass
from enum import Enum, auto

class BootMode(Enum):
    COLD_BOOT = 'Cold Boot (Full Init)'
    INSTANT_ON = 'Instant-On (RAM Snapshot)'
    RECOVERY = 'Recovery Safe Mode'
    VIRTUAL_VM = 'VM Orchestration Boot'