from ..interfaces.base_sovereign import SovereignModule
from ..interfaces.command_interfaces import ICommander, ICommand

class SovereignCommander(SovereignModule, ICommander):
    """
    Sovereign Commander.
    Centralized router for system-level actions (Command Pattern).
    """
    def __init__(self):
        super().__init__("SOVEREIGN_COMMANDER")
        self._commands = {}

    def register_command(self, name: str, command: ICommand):
        print(f"[COMMANDER] Registering logic: {name}")
        self._commands[name] = command

    def dispatch(self, name: str, *args, **kwargs):
        cmd = self._commands.get(name)
        if cmd:
            print(f"[COMMANDER] Dispatching: {name}")
            return cmd.execute(*args, **kwargs)
        raise KeyError(f"Command '{name}' not found.")

    def execute(self, action, *args, **kwargs):
        """Standard ISovereign contract."""
        if action == "DISPATCH":
            return self.dispatch(*args, **kwargs)
        return f"COMMANDER_READY"

    def initialize(self):
        print("[COMMANDER] Command Dispatcher Online.")

    def shutdown(self):
        self._commands.clear()
        print("[COMMANDER] Command Dispatcher Offline.")

    def health_check(self) -> bool:
        return True

def get_commander() -> SovereignCommander:
    return SovereignCommander()
