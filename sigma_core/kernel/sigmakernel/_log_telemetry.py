"""
Auto-split from sigma_core\kernel.py — SigmaKernel._log_telemetry
"""

import sys
import os
import re
import threading
import time
import hashlib
import subprocess
import platform
import ctypes
import random
import contextlib
from typing import Dict, List, Any, Optional, Generator
from contextlib import contextmanager
from .system.config import SigmaConfig
from .system.event_bus import EventBus
from .system.registry import ModuleRegistry
from .system.ledger import SovereignLedger
from .system.cache import SigmaCache
from .security.integrity import IntegrityGuard
from .ui.customizer import SovereignCustomizer
from .security.vanguard import NetworkVanguard
from .system.guardian import SigmaGuardian
from .system.loader import SigmaModuleLoader
from .hal.polyglot_loader import SigmaPolyglot
from .manifest import CORE_SYSTEM_MODULES, ECOSYSTEM_APPS



class SigmaKernel:
    def _log_telemetry(self, op: str, latency: float):
        if self.visualizer:
            try:
                self.visualizer.report_latency(op, latency)
            except:
                pass
        if latency > 100.0 and self.ledger:
            self.ledger.log_event('TELEMETRY', f'SLOW_OP_{op}', f'{latency:.2f}ms', 'WARN')
