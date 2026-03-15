import time
import os
from abc import ABC, abstractmethod
from sigma_core.interfaces.base_sovereign import SovereignModule


def add_model(self, name: str, url: str, selector: str):
    self._models[name] = {'url': url, 'selector': selector}
    print(f'[OMNI-PROMPT] Registered Model: {name}')