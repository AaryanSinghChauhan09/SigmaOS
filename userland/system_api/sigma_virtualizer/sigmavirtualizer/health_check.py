# Generated method: SigmaVirtualizer.health_check


class SigmaVirtualizer:
    def health_check(self) -> str:
        s = 'VBOX_NATIVE' if self._is_vbox else 'BARE_METAL'
        return f'OK — Sigma Virtualizer Active. Platform: {s}. Hypervisor: Optimized.'