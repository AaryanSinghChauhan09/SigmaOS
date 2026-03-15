# Generated class core: ComputeState
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import time

@dataclass
class ComputeState:
    cpu_usage: float
    ram_available: float
    mesh_nodes_online: int