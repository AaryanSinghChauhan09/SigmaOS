# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import hashlib
import os

def verify_system_state():
    validator = MerkleIntegrityValidator('.')
    root_hash = validator.audit_entire_fleet()
    return root_hash