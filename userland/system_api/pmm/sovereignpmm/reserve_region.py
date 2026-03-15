# Generated method: SovereignPMM.reserve_region


class SovereignPMM:
    def reserve_region(self, start_addr: int, size: int):
        start_block = start_addr // self.block_size
        num_blocks = size // self.block_size
        for i in range(start_block, start_block + num_blocks):
            byte_idx = i // 8
            bit_idx = i % 8
            if byte_idx < len(self.bitmap):
                self.bitmap[byte_idx] |= 1 << bit_idx
                self.used_blocks += 1