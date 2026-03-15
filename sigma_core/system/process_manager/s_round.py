"""
Auto-split from sigma_core\system\process_manager.py — s_round
"""

import time
import uuid
import random
import threading
from typing import Dict, List, Any, Optional
from enum import Enum
from dataclasses import dataclass, field



def s_round(val: float, digits: int=1) -> float:
    try:
        return float(f'{val:.{digits}f}')
    except:
        return float(val)
