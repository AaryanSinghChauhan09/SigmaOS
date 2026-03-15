# Generated method: SigmaTitanCapture.enable_webcam_overlay
from sigma_core.system.sovereign_app import SovereignApp

class SigmaTitanCapture:
    def enable_webcam_overlay(self, shape='Circle'):
        """Instant webcam overlay with local background removal."""
        return self._call_service('Vision_Engine', 'Webcam_Inject', shape=shape)