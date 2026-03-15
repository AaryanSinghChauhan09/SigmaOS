# Generated method: SigmaOmniBrowser.switch_engine
import random
from sigma_core.system.sovereign_app import SovereignApp

class SigmaOmniBrowser:
    def switch_engine(self, engine_name):
        """
                Dynamically switches the rendering engine.
                Supported: 'Chromium', 'Gecko', 'WebKit', 'Comet-Lite'.
                """
        engines = ['Chromium', 'Gecko', 'WebKit', 'Comet-Lite']
        if engine_name in engines:
            self.engine = engine_name
            return f'OmniBrowser: Engine hot-swapped to {engine_name}.'
        return 'Error: Engine not supported.'