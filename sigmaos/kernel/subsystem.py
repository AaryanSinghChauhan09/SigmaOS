"""
SigmaOS Subsystem Manager
Handles the dynamic loading and unloading of isolated OS shards.
Ensures that modules like networking, media, and security are only active when needed.
"""
from typing import Dict, Any, Type
import importlib

class Subsystem:
    """Base class for all SigmaOS modular subsystems."""
    def __init__(self, name: str):
        self.name = name
        self.is_loaded = False

    def load(self):
        print(f"[Subsystem] Loading {self.name}...")
        self.is_loaded = True

    def unload(self):
        print(f"[Subsystem] Unloading {self.name}...")
        self.is_loaded = False

class SubsystemManager:
    def __init__(self):
        self.registry: Dict[str, Subsystem] = {}

    def register(self, name: str, subsystem: Subsystem):
        self.registry[name] = subsystem

    def load_subsystem(self, name: str):
        if name in self.registry:
            self.registry[name].load()
        else:
            print(f"[SubsystemManager] Error: Subsystem {name} not found.")

    def unload_subsystem(self, name: str):
        if name in self.registry:
            self.registry[name].unload()
        else:
            print(f"[SubsystemManager] Error: Subsystem {name} not found.")

    def list_active(self):
        return [name for name, sub in self.registry.items() if sub.is_loaded]

# Canonical Global Manager
manager = SubsystemManager()
