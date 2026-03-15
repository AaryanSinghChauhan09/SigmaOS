# Generated class core: FormFactor
from enum import Enum
from dataclasses import dataclass

class FormFactor(Enum):
    MOBILE = 'Vertical_Compact (Touch-First)'
    TABLET = 'Flexible_Split (Hybrid-Touch)'
    LAPTOP = 'Clamshell_Dense (Keyboard-First)'
    DESKTOP = 'Ultra_Wide (Multi-Window)'