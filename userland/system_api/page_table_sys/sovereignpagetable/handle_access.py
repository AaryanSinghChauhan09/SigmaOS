# Generated method: SovereignPageTable.handle_access
from dataclasses import dataclass
from enum import Enum

class SovereignPageTable:
    def handle_access(self, virt_addr: int, write: bool=False):
        """Syscall/Trap handler for memory access."""
        if virt_addr not in self.entries:
            return {'status': 'PAGE_FAULT', 'action': 'DISK_LOAD'}
        entry = self.entries[virt_addr]
        if entry.status == PageStatus.GHOST:
            entry.status = PageStatus.PRESENT
            if write:
                entry.writable = True
            print(f'[MMU] Ghost Hit! Promoted {hex(virt_addr)} to PRESENT.')
            return {'status': 'GHOST_HIT', 'latency': '0ms'}
        return {'status': 'SUCCESS', 'latency': '0.1ns'}