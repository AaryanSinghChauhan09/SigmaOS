"""
Auto-split from sigma_core\kernel.py — SigmaKernel.startup
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
    def startup(self):
        if self.ledger:
            self.ledger.log_event('BOOT', 'FINAL_INIT', 'SigmaOS Kernel Ready.')
        if self.syncer:
            self.syncer.start_service()
        auto_loader = self.registry.get('auto_load')
        if auto_loader:
            auto_loader.process_queue()
        self.pulse_system()
