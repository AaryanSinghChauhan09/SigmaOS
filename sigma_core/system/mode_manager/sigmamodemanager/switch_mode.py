"""
Auto-split from sigma_core\system\mode_manager.py — SigmaModeManager.switch_mode
"""

from typing import Dict, List, Any, Callable, Optional
import time



class SigmaModeManager:
    def switch_mode(self, mode_name: str) -> Dict:
        """USP: Atomic mode switching with kernel re-profiling and routine execution."""
        if mode_name not in self._modes:
            return {'Error': f"Mode '{mode_name}' not recognized."}
        old_mode = self._current_mode
        old_profile = self._modes[old_mode]
        new_profile = self._modes[mode_name]
        exit_list: Any = old_profile.get('Routines_On_Exit', [])
        if not isinstance(exit_list, list):
            exit_list = []
        exit_routine_results = self._apply_routines(exit_list, 'exit')
        if hasattr(self.kernel, 'prewarmer') and self.kernel.prewarmer:
            self.kernel.prewarmer.purge_cold_apps()
        self._current_mode = mode_name
        if self.kernel and hasattr(self.kernel, 'bus'):
            self.kernel.bus.emit('mode.change', {'mode': mode_name})
        if self.kernel and hasattr(self.kernel, 'registry'):
            config = self.kernel.registry.get('config')
            if config and hasattr(config, 'apply_mode'):
                config.apply_mode(mode_name.lower())
        tune_status = self._apply_tuning(new_profile)
        enter_list: Any = new_profile.get('Routines_On_Enter', [])
        if not isinstance(enter_list, list):
            enter_list = []
        enter_routine_results = self._apply_routines(enter_list, 'enter')
        return {'From': old_mode, 'To': mode_name, 'Performance_Profile': new_profile, 'Kernel_Tuning': tune_status, 'Exit_Routines_Status': exit_routine_results, 'Enter_Routines_Status': enter_routine_results, 'Status': 'READY'}
