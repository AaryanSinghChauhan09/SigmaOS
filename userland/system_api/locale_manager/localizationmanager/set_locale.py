# Generated method: LocalizationManager.set_locale
import time
import random
from typing import Dict, List, Any

class LocalizationManager:
    def set_locale(self, locale: str) -> bool:
        """TC-LOC-004: Switch locale without system reboot."""
        if locale not in self._supported_locales:
            return False
        self.current_locale = locale
        self._rtl_active = locale in ('ar-SA', 'he-IL')
        self.kernel.bus.emit('locale.changed', {'locale': locale, 'rtl': self._rtl_active})
        return True