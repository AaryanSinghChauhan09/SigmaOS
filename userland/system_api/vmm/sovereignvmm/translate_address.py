# Generated method: SovereignVMM.translate_address


class SovereignVMM:
    def translate_address(self, virtual_addr: int) -> int:
        """USP: Hardware-Parity Address Translation."""
        if not self.is_paging_enabled:
            return virtual_addr
        dir_idx = virtual_addr >> 22 & 1023
        table_idx = virtual_addr >> 12 & 1023
        offset = virtual_addr & 4095
        dir_entry = self.page_directory.get(dir_idx)
        if dir_entry:
            table_entry = dir_entry.get(table_idx)
            if table_entry:
                return table_entry['phys'] + offset
        if hasattr(self.kernel, 'interrupts'):
            self.kernel.interrupts.trigger_interrupt(14, virtual_addr)
        return -1