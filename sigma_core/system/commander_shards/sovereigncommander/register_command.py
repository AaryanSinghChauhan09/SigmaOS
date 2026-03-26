from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.command_interfaces import ICommander, ICommand

from ._base import SovereignCommander

class SovereignCommander:
    def register_command(self, name: str, command: ICommand):
        print(f'[COMMANDER] Registering logic: {name}')
        self._commands[name] = command