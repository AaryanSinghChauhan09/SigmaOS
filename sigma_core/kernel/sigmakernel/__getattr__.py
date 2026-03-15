"""
Auto-split from sigma_core\kernel.py — SigmaKernel.__getattr__
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
    def __getattr__(self, name: str) -> Any:
        """Dynamic Shard Accessor with fallback safety."""
        aliases = {'perf': 'perf', 'net_guard': 'net_guard', 'fs': 'fs', 'modes': 'modes', 'rituals': 'rituals', 'bridge': 'bridge', 'search': 'sovereign_search'}
        key = aliases.get(name, name)
        module = self.registry.get(key)
        if module:
            return module
        shard_attrs = ['aura', 'vector_memory', 'governance', 'vibe_scheduler', 'shifter', 'mesh', 'airgap', 'zk_sync', 'universal', 'aether_grid', 'troubleshooter', 'hypervisor', 'latency_engine', 'agent_bridge', 'eco_manager', 'visualizer', 'accelerator', 'brain', 'agent', 'pulse', 'telemetry', 'sovereign_agent', 'repair_engine', 'ledger']
        if name in shard_attrs:
            return None
        raise AttributeError(f"'SigmaKernel' object has no attribute '{name}'")
