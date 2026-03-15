# Generated class core: ActionNode
import time
import uuid
import threading
from typing import List, Dict, Any, Optional
from dataclasses import dataclass, field

@dataclass
class ActionNode:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    action: str = ''
    params: Dict[str, Any] = field(default_factory=dict)
    retry_policy: int = 3
    rollback_action: Optional[str] = None