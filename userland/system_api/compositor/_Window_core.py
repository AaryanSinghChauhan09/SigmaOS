# Generated class core: Window
from dataclasses import dataclass, field
from typing import List

@dataclass
class Window:
    id: str
    owner_pid: int
    x: int
    y: int
    width: int
    height: int
    opacity: int = 255
    z_order: int = 0
    buffer: list = field(default_factory=list)