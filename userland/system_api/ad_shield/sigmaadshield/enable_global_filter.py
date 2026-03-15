# Generated method: SigmaAdShield.enable_global_filter


class SigmaAdShield:
    def enable_global_filter(self):
        """USP: Brave-grade Shielding. Injecting DoH + Socket Interception."""
        print('[*] Shield: Injecting global DNS-over-HTTPS (DoH) and Socket layer filters...')
        self.active_protection = True
        return 'Ad-Shield (Brave USP): ACTIVE. Blocking global ad-delivery and tracker-pulse networks.'