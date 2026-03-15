from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.kernel.kernel_states import BootingState, RunningState, HaltedState

from ._base import SigmaKernel

class SigmaKernel:
    def set_state(self, state):
        print(f'[KERNEL] Transitioning to state: {state.__class__.__name__}')
        self._state = state
        if self._power:
            if isinstance(state, RunningState):
                self._power.set_mode('OPTIMIZED')