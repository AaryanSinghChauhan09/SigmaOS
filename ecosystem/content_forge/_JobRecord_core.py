# Generated class core: JobRecord
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import time

@dataclass
class JobRecord:
    id: str
    action: str
    status: str
    details: Dict[str, Any]