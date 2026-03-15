# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import hashlib
import os

class MerkleIntegrityValidator:
    """
    Builds a Merkle Tree from the myriad of small modules to ensure system integrity. 
    If a single file is tampered with, the root hash changes.
    """