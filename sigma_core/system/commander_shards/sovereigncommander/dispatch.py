from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.command_interfaces import ICommander, ICommand

from ._base import SovereignCommander

class SovereignCommander:
    def dispatch(self, name: str, *args, **kwargs):
        cmd = self._commands.get(name)
        if cmd:
            print(f'[COMMANDER] Dispatching: {name}')
            return cmd.execute(*args, **kwargs)
        raise KeyError(f"Command '{name}' not found.")