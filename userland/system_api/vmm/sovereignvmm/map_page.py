# Generated method: SovereignVMM.map_page


class SovereignVMM:
    def map_page(self, virtual_addr: int, physical_addr: int, flags: int=3):
        """USP: Recursive Paging. Maps a 4KB virtual page to a physical block."""
        dir_idx = virtual_addr >> 22 & 1023
        table_idx = virtual_addr >> 12 & 1023
        if dir_idx not in self.page_directory:
            self.page_directory[dir_idx] = {}
        self.page_directory[dir_idx][table_idx] = {'phys': physical_addr, 'flags': flags}
        return f'VMM: Mapped 0x{virtual_addr:08x} -> 0x{physical_addr:08x} [Flags: {hex(flags)}]'