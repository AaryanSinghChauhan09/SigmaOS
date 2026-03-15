# Generated class core: PCIBar
from dataclasses import dataclass, field
from typing import List, Optional

@dataclass
class PCIBar:
    base: int
    size: int
    is_io: bool = False