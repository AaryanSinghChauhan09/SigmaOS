# Generated method: SovereignInterruptManager._handle_syscall
from enum import Enum, auto
import secrets

class SovereignInterruptManager:
    def _handle_syscall(self, call_data):
        if hasattr(self.kernel, 'syscall_gateway'):
            return self.kernel.syscall_gateway.execute(call_data)
        return {'error': 'Syscall gateway not initialized'}