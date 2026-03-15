# Generated method: SovereignPMM.alloc_block


class SovereignPMM:
    def alloc_block(self) -> int:
        """Finds the first free block in the bitmap and marks it used."""
        for i in range(len(self.bitmap)):
            if self.bitmap[i] != 255:
                for bit in range(8):
                    if not self.bitmap[i] & 1 << bit:
                        self.bitmap[i] |= 1 << bit
                        self.used_blocks += 1
                        return i * 8 + bit
        return -1