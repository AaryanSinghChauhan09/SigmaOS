# Generated method: SigmaTitanCapture.extract_text_from_region
from sigma_core.system.sovereign_app import SovereignApp

class SigmaTitanCapture:
    def extract_text_from_region(self):
        """Native Screen OCR (no cloud)."""
        return self._call_service('AI_Engine', 'Screen_Grab_Text')