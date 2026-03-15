# Generated method: SovereignPMM.free_block


class SovereignPMM:
    def free_block(self, block_index: int):
        byte_idx = block_index // 8
        bit_idx = block_index % 8
        if self.bitmap[byte_idx] & 1 << bit_idx:
            self.bitmap[byte_idx] &= ~(1 << bit_idx)
            self.used_blocks -= 1