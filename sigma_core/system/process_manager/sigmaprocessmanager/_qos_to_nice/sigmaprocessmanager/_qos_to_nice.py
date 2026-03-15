# Generated method: SigmaProcessManager._qos_to_nice
import time
import uuid
import random
import threading
from typing import Dict, List, Any, Optional
from enum import Enum
from dataclasses import dataclass, field

class SigmaProcessManager:
    @staticmethod
    def _qos_to_nice(qos: QoSClass) -> int:
        return {QoSClass.REALTIME: -20, QoSClass.USER_INTERACTIVE: -10, QoSClass.USER_INITIATED: 0, QoSClass.UTILITY: 10, QoSClass.BACKGROUND: 19}.get(qos, 0)