# Generated method: SovereignVMM.enable_paging


class SovereignVMM:
    def enable_paging(self):
        self.is_paging_enabled = True
        return 'CR0.PG bit set. Virtual Addressing ACTIVE.'