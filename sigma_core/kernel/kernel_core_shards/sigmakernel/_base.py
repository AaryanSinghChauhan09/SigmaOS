from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.kernel.kernel_states import BootingState, RunningState, HaltedState


class SigmaKernel(SovereignModule):
    __slots__ = ('_page_table', '_power', '_state', '_threads')
    '\n    Sovereign Kernel - Core Execution Engine.\n    Implements State Pattern and Composition.\n    '