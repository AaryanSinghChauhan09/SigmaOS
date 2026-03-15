import time
import os
from abc import ABC, abstractmethod
from sigma_core.interfaces.base_sovereign import SovereignModule
from .sovereignomniprompt._base import SovereignOmniPrompt

def get_omni_prompt():
    return SovereignOmniPrompt()