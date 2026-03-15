# Generated method: SigmaConfig.apply_mode


class SigmaConfig:
    def apply_mode(self, mode_name: str):
        """
            USP: Atomic Environment Profile Sync.
            Maps the higher-level OS 'Mode' to internal configuration and vibes.
            Called by SigmaModeManager for deep-kernel/userland alignment.
            """
        mode_name = mode_name.lower()
        if mode_name == 'gaming':
            self.apply_vibe('Gamer')
            self.HIGH_PERFORMANCE_IO = True
        elif mode_name == 'programmer' or mode_name == 'ai_engineer':
            self.apply_vibe('Enterprise')
            self.ENABLE_AGENTIC = True
        elif mode_name == 'standard':
            self.apply_vibe('Enterprise')
        elif mode_name == 'bare_minimum':
            self.apply_vibe('Minimalist')
            self.ENABLE_GUI = False
        self.set('ACTIVE_MODE', mode_name)
        print(f"[DNA] System State re-calculated for mode '{mode_name}'.")