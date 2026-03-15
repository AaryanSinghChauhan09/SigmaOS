# Generated method: SigmaCAAT.evaluate_context
from enum import Enum
import time
import random
from dataclasses import dataclass, field

class SigmaCAAT:
    def evaluate_context(self) -> dict:
        """
                The 'Sense & Decide' phase. 
                Evaluates current sensors to determine the optimal OS operating context.
                """
        self._stats['inferences'] += 1
        old_context = self._current_context
        if self._sensors['active_window'] in ['Visual Studio', 'Figma', 'Excel']:
            new_context = ContextState.WORK
        elif self._sensors['cpu_load'] > 60 and self._sensors['active_window'] == 'Game':
            new_context = ContextState.GAMING
        elif self._sensors['battery_pct'] < 30 or self._sensors['grid_carbon_intensity'] > 300:
            new_context = ContextState.TRAVEL
        else:
            new_context = ContextState.WELLNESS
        if new_context != old_context:
            self._current_context = new_context
            action = self._apply_context_profile(new_context)
            self._log_action(f'Context shifted to {new_context.name}', action)
            return {'changed': True, 'old': old_context.name, 'new': new_context.name, 'action_taken': action, 'message': f'CAAT: Context seamlessly shifted to {new_context.value}.'}
        return {'changed': False, 'current': new_context.name, 'message': f'CAAT: Maintaining existing context ({new_context.value}).'}