import time
import os
from abc import ABC, abstractmethod
from sigma_core.interfaces.base_sovereign import SovereignModule


@abstractmethod
def distribute(self, prompt: str, model_name: str):
    pass