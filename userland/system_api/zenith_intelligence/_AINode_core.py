# Generated class core: AINode
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any
import time
import random
import json
import os
from pathlib import Path

@dataclass
class AINode:
    name: str
    url: str
    category: str = 'AI'
    color: str = '#6366f1'
    used_percent: int = 0
    quota_limit: int = 100
    status: str = 'ONLINE'