from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.kernel.kernel_states import BootingState, RunningState, HaltedState

from ._base import SigmaKernel

class SigmaKernel:
    def shutdown(self):
        self.set_state(HaltedState())
        self.status = 'SHUTTING_DOWN'
        if self._power:
            self._power.set_mode('LOW_POWER')
        print('[KERNEL] Safe shutdown sequence complete.')