# Generated class core: BootProfile
import time
import uuid
import hashlib
from dataclasses import dataclass
from enum import Enum, auto

@dataclass
class BootProfile:
    profile_id: str
    name: str
    kernel_opt: str
    auto_load: list[str]
    ai_desc: str