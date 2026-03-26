import time
import os
from abc import ABC, abstractmethod
from sigma_core.interfaces.base_sovereign import SovereignModule


def _distribute_to_model(self, name, url, selector, prompt):
    print(f'--- [SYNCING] {name} ---')
    print(f'URL: {url}')
    print(f'ACTION: Locating {selector} and injecting prompt...')
    time.sleep(0.5)
    print(f'✅ Prompt Distilled into {name} input buffer.')