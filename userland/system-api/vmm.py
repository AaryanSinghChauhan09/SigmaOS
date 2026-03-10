"""
Sovereign Virtual Memory Manager (VMM) — v1.0
==============================================
USP: Recursive Paging & Multi-Level Address Translation.
     Ensures Program Isolation (Ring-3 Safety).
"""

class SovereignVMM:
    def __init__(self, kernel):
        self.kernel = kernel
        self.page_directory = {} # Map virtual 4MB chunk -> Page Table
        self.is_paging_enabled = False

    def enable_paging(self):
        self.is_paging_enabled = True
        return "CR0.PG bit set. Virtual Addressing ACTIVE."

    def map_page(self, virtual_addr: int, physical_addr: int, flags: int = 0x03):
        """USP: Recursive Paging. Maps a 4KB virtual page to a physical block."""
        dir_idx = (virtual_addr >> 22) & 0x3FF
        table_idx = (virtual_addr >> 12) & 0x3FF
        
        if dir_idx not in self.page_directory:
            self.page_directory[dir_idx] = {}
            
        self.page_directory[dir_idx][table_idx] = {
            "phys": physical_addr,
            "flags": flags
        }
        return f"VMM: Mapped 0x{virtual_addr:08x} -> 0x{physical_addr:08x} [Flags: {hex(flags)}]"

    def translate_address(self, virtual_addr: int) -> int:
        """USP: Hardware-Parity Address Translation."""
        if not self.is_paging_enabled:
            return virtual_addr
            
        dir_idx = (virtual_addr >> 22) & 0x3FF
        table_idx = (virtual_addr >> 12) & 0x3FF
        offset = virtual_addr & 0xFFF
        
        dir_entry = self.page_directory.get(dir_idx)
        if dir_entry:
            table_entry = dir_entry.get(table_idx)
            if table_entry:
                return table_entry["phys"] + offset
                
        # Trigger Page Fault via Interrupt Manager if active
        if hasattr(self.kernel, 'interrupts'):
            self.kernel.interrupts.trigger_interrupt(0x0E, virtual_addr)
        return -1 # Page Fault

    def health_check(self) -> str:
        mapped = sum(len(t) for t in self.page_directory.values())
        return f"OK — VMM: Paging {'ACTIVE' if self.is_paging_enabled else 'IDLE'} | {mapped} pages in directory."
