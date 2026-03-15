# Generated method: SigmaCustomizer.get_ui_manifest
import json
import random
import os

class SigmaCustomizer:
    def get_ui_manifest(self) -> dict:
        return {'active_theme': self.active_theme, 'styles': self._styles, 'stats': self._stats, 'capabilities': ['Mica', 'MaterialYou', 'AtomShaders', 'VectorMorph']}