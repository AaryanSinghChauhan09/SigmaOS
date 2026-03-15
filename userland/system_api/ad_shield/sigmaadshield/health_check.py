# Generated method: SigmaAdShield.health_check


class SigmaAdShield:
    def health_check(self) -> str:
        status = 'PROTECTED' if self.active_protection else 'EXPOSED'
        return f'OK — Sigma Ad-Shield: {status} | Global Blocklist: {self.blocklist_count} nodes.'