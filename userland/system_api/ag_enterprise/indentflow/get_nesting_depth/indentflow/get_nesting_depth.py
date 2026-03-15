# Generated method: IndentFlow.get_nesting_depth
import os
import re
import json
import time
from typing import List, Dict, Any, Optional

class IndentFlow:
    def get_nesting_depth(self, code: str) -> List[int]:
        depths = []
        current = 0
        for line in code.splitlines():
            current += line.count('{') - line.count('}')
            depths.append(max(0, current))
        return depths