from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.command_interfaces import ICommander, ICommand

from ._base import SovereignCommander

class SovereignCommander:
    def execute(self, action, *args, **kwargs):
        """Standard ISovereign contract."""
        if action == 'DISPATCH':
            return self.dispatch(*args, **kwargs)
        return f'COMMANDER_READY'