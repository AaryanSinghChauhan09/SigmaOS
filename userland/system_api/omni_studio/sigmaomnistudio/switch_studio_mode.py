# Generated method: SigmaOmniStudio.switch_studio_mode
from typing import Dict
import time

class SigmaOmniStudio:
    def switch_studio_mode(self, mode: str) -> Dict:
        """Morphs the application into the desired professional toolkit."""
        if mode not in self.modes:
            return {'status': 'ERROR', 'message': f"Mode '{mode}' not supported in Omni-Studio."}
        self.active_mode = mode
        self.project_state = {'started': time.time(), 'unsaved_changes': False}
        w_man = self.kernel.registry.get('omni_work')
        if w_man and mode in ['Programmer', 'Video Editor', 'Designer']:
            w_man.apply_workspace(mode.split(' ')[-1])
        config = self.modes[mode]
        return {'status': 'MORPHED_SUCCESS', 'mode': mode, 'replaces': config['competitor_replaced'], 'features_loaded': config['active_panels'], 'usps_activated': config['usps'], 'message': f"Omni-Studio morphed into '{mode}' mode. Replaces {config['competitor_replaced']}."}