"""
Auto-split from userland\system_api\network_stack.py — SigmaNetworkStack.get_qos_stats
"""

import time
import uuid
import hashlib
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaNetworkStack:
    def get_qos_stats(self) -> dict:
        by_priority = {p.name: 0 for p in FlowPriority}
        for flow in self._flows.values():
            by_priority[flow.priority.name] += 1
        return {'total_flows': len(self._flows), 'by_priority': by_priority, 'message': f'AdaptiveQoS: {len(self._flows)} active flows classified.'}
