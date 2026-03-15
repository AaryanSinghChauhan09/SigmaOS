# Generated class core: UIState
from enum import Enum
from dataclasses import dataclass

@dataclass
class UIState:
    form_factor: FormFactor
    scaling: float
    nav_style: str
    active_space: str