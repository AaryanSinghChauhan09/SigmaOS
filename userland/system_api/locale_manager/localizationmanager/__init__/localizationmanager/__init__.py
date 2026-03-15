# Generated method: LocalizationManager.__init__
import time
import random
from typing import Dict, List, Any

class LocalizationManager:
    def __init__(self, kernel):
        self.kernel = kernel
        self.current_locale = 'en-IN'
        self._supported_locales = ['en-IN', 'hi-IN', 'kn-IN', 'ta-IN', 'ar-SA', 'he-IL']
        self._rtl_active = False