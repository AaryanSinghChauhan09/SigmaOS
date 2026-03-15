# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ctypes
import os

def get_native_bridge():
    return NativePerformanceBridge()