# Generated class core: Task
from dataclasses import dataclass, field
import time

@dataclass
class Task:
    id: int
    name: str
    priority: int
    state: str = 'READY'
    context: dict = field(default_factory=lambda: {'EAX': 0, 'EBX': 0, 'ESP': 0, 'EIP': 0})
    runtime_ms: float = 0.0