# Generated class core: SigmaWindow
from dataclasses import dataclass, field
import uuid
from sigma_core.system.interfaces import SigmaModuleBase

@dataclass
class SigmaWindow:
    id: str
    title: str
    x: int
    y: int
    w: int
    h: int
    z_index: int = 0
    minimized: bool = False
    maximized: bool = False
    is_active: bool = False
    type: str = 'Generic'