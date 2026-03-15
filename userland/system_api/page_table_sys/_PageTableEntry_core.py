# Generated class core: PageTableEntry
from dataclasses import dataclass
from enum import Enum

@dataclass
class PageTableEntry:
    frame_addr: int
    status: PageStatus
    writable: bool = False
    accessed: bool = False