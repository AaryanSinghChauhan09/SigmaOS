"""
SigmaOS Kernel v2.1 — Apex Core (Modular & Resilient)
======================================================
USP: Ultra-lean orchestration layer. All logic delegated to autonomous shards.
Satisfies mandates: Encapsulation, Abstraction, Loose Coupling, High Cohesion.
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
from typing import Dict, List, Any, Optional

from .system.config import SigmaConfig
from .system.event_bus import EventBus
from .system.registry import ModuleRegistry
from .system.ledger import SovereignLedger
from .system.cache import SigmaCache
from .security.integrity import IntegrityGuard
from .ui.customizer import SovereignCustomizer
from .security.vanguard import NetworkVanguard
from .system.loader import SigmaModuleLoader
from .hal.polyglot_loader import SigmaPolyglot
from .manifest import CORE_SYSTEM_MODULES, ECOSYSTEM_APPS

class SigmaKernel:
    """
    Sovereign Kernel v2.1 — Orchestrating the Apex Shard Grid.
    """

    def __init__(self, auto_load: bool = True):
        self.cfg = SigmaConfig()
        self.bus = EventBus()
        self.registry = ModuleRegistry()
        self.loader = SigmaModuleLoader(self)
        self.ledger = SovereignLedger()
        
        # Core Platform Services
        self.cache = SigmaCache(self)
        self.integrity = IntegrityGuard(self)
        self.customizer = SovereignCustomizer(self)
        self.vanguard_engine = NetworkVanguard(self)
        
        # Register Core
        self.registry.register("cache", self.cache)
        self.registry.register("integrity", self.integrity)
        self.registry.register("customizer", self.customizer)
        self.registry.register("vanguard", self.vanguard_engine)
        
        self.os_name = self.cfg.OS_NAME
        self.version = self.cfg.VERSION
        self._root = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))

        # Bootstrap: Run native priority layers
        self._low_level_init()

        if auto_load:
            self._hydrate_apex_grid()

    def _low_level_init(self):
        """Win32/POSIX Low-Level Memory & Priority Locking."""
        if platform.system() == "Windows":
            try:
                kernel32 = getattr(ctypes, "windll", None)
                if kernel32:
                    current_proc = kernel32.kernel32.GetCurrentProcess()
                    kernel32.kernel32.SetPriorityClass(current_proc, 0x00000080) # HIGH
            except Exception: pass
            SigmaPolyglot.run_priority_layer("BOOTLOADER", "boot")
        elif platform.system() == "Linux":
            try: os.nice(-20)
            except: pass

    def _hydrate_apex_grid(self):
        """USP: Parallel Apex Hydration using the Manifest."""
        print(f"[KERNEL] Initiating Apex Hydration [v{self.version}]...")
        
        # 1. Load System Shards in Parallel
        self.loader.load_modules_parallel(CORE_SYSTEM_MODULES)
        
        # 2. Load Ecosystem Apps
        self.loader.load_modules_parallel(ECOSYSTEM_APPS)
        
        # 3. Lifecycle Start: Iterate through all registered services
        for key in self.registry.list_modules():
            service = self.registry.get(key)
            if service and hasattr(service, "start_service"):
                try:
                    res = service.start_service()
                    if "ERR" in str(res):
                        print(f"  [!] {key} Initialization Warning: {res}")
                except Exception as e:
                    print(f"  [!] {key} Crash on Startup: {e}")

        # 4. Final Shell/Aura Trigger
        if self.aura:
            self.aura.apply_aura("DeepSpace")
        print(f"[KERNEL] Grid Online. All USPs Hydrated.")

    def __getattr__(self, name: str) -> Any:
        """
        USP: Dynamic Shard Accessor.
        Proxies kernel attribute access to the module registry.
        Fulfillment of 'Abstraction' and 'Loose Coupling'.
        """
        # Mapping legacy names to registry keys if needed
        aliases = {
            "perf": "perf", "net_guard": "net_guard", "fs": "fs",
            "modes": "modes", "rituals": "rituals", "bridge": "bridge",
            "search": "sovereign_search"
        }
        key = aliases.get(name, name)
        module = self.registry.get(key)
        if module:
            return module
        raise AttributeError(f"'SigmaKernel' object has no attribute '{name}'")

    def self_healing_recovery(self) -> str:
        """Sovereign Repair Engine. Restores integrity from evidence vault."""
        repair = self.registry.get("repair_engine")
        return repair.repair("SYSTEM", "Integrity Breach") if repair else "REPAIR_OFFLINE"

    def health_check(self) -> dict:
        return {
            "status": "ONLINE",
            "version": self.version,
            "shards": self.registry.health_check()
        }

if __name__ == "__main__":
    k = SigmaKernel()
    print(k.health_check())
