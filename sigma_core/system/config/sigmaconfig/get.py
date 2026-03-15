# Generated method: SigmaConfig.get


class SigmaConfig:
    def get(self, key, default=None):
        """Get a configuration value"""
        if key in self._custom_settings:
            return self._custom_settings[key]
        return getattr(self, key, default)