import time
import os
from abc import ABC, abstractmethod
from sigma_core.interfaces.base_sovereign import SovereignModule


def health_check(self) -> bool:
    return True