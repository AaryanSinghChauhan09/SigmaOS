# Generated class core: ContainerState
import time
import uuid
from dataclasses import dataclass
from enum import Enum, auto

class ContainerState(Enum):
    STOPPED = 'stopped'
    STARTING = 'starting'
    RUNNING = 'running'
    PAUSED = 'paused'
    FROZEN = 'frozen'