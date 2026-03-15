# Generated method: SigmaKernel._register_core_services
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
    def _register_core_services(self):
        """Unified registry for Ring 0 system services."""
        self.registry.register('cache', self.cache)
        self.registry.register('integrity', self.integrity)
        self.registry.register('customizer', self.customizer)
        self.registry.register('vanguard', self.vanguard_engine)
        self.registry.register('guardian', self.guardian)
        self.registry.register('crusher', self.crusher)
        self.registry.register('syncer', self.syncer)
        self.registry.register('web_syncer', self.syncer)
        self.registry.register('distillator', self.distillator)