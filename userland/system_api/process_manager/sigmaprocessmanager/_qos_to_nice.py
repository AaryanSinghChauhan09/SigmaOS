"""
Auto-split from userland\system_api\process_manager.py — SigmaProcessManager._qos_to_nice
"""

import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaProcessManager:
    @staticmethod
    def _qos_to_nice(qos: QoSClass) -> int:
        return {QoSClass.REALTIME: -20, QoSClass.USER_INTERACTIVE: -10, QoSClass.USER_INITIATED: 0, QoSClass.UTILITY: 10, QoSClass.BACKGROUND: 19}[qos]
