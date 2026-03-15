# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import os
import hashlib

def run_evolution_scan():
    engine = SelfEvolvingEngine('.')
    return engine.evolution_proposal()