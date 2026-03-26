import time
import os
from abc import ABC, abstractmethod
from sigma_core.interfaces.base_sovereign import SovereignModule


def distribute_prompt(self, prompt: str, model_list: list=None):
    target_models = model_list if model_list else list(self._models.keys())
    print(f"🚀 Distributing Prompt To: {', '.join(target_models)}")
    for name in target_models:
        if name in self._models:
            model = self._models[name]
            self._distribute_to_model(name, model['url'], model['selector'], prompt)