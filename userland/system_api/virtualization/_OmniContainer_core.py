# Generated class core: OmniContainer
import time
import uuid
from dataclasses import dataclass
from enum import Enum, auto

@dataclass
class OmniContainer:
    container_id: str
    name: str
    guest_os: GuestOS
    state: ContainerState = ContainerState.STOPPED
    cpu_cores: int = 1
    ram_mb: float = 512.0
    gui_projected: bool = True
    cloud_burst: bool = False
    boot_time_ms: float = 0.0