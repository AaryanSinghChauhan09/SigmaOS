# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
from sigma_core.security.resilience_guard import resilient_module
import os
import importlib.util
import sys

@resilient_module
def get_registry():
    return UniversalRegistry(os.getcwd())