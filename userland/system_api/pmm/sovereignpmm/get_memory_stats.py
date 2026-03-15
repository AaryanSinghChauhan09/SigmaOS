# Generated method: SovereignPMM.get_memory_stats


class SovereignPMM:
    def get_memory_stats(self) -> dict:
        free_blocks = self.total_blocks - self.used_blocks
        return {'Total_RAM': f'{self.total_blocks * self.block_size // (1024 * 1024)} MB', 'Used_RAM': f'{self.used_blocks * self.block_size // 1024} KB', 'Free_RAM': f'{free_blocks * self.block_size // 1024} KB', 'Utilization': f'{self.used_blocks / self.total_blocks:.2%}'}