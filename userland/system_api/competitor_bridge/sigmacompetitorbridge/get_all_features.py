"""
Auto-split from userland\system_api\competitor_bridge.py — SigmaCompetitorBridge.get_all_features
"""



class SigmaCompetitorBridge:
    def get_all_features(self):
        """Aggregates all competitor-equivalent features available in SigmaOS."""
        return {'Windows': self.windows_features(), 'macOS': self.macos_features(), 'Linux': self.linux_features(), 'ChromeOS': self.chromeos_features(), 'Mobile': self.mobile_features()}
