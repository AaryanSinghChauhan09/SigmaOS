# Generated class core: InputMode
from enum import Enum
from dataclasses import dataclass
import uuid

class InputMode(Enum):
    TOUCH = 'Digitizer (Absolute)'
    MOUSE = 'Pointer (Relative)'
    STYLUS = 'Pressure-Sensitive'
    BCI = 'Neural-Input (Direct)'