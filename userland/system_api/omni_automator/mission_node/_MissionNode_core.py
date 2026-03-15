# Generated class core: MissionNode
from dataclasses import dataclass, field
from typing import Dict, Any, Optional

@dataclass
class MissionNode:
    id: str
    name: str
    node_type: str
    params: Dict[str, Any] = field(default_factory=dict)
    next_node_id: Optional[str] = None
    execution_time_ms: float = 0.0