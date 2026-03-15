# Generated method: AILifecyclePage._execute_ai_next
import tkinter as tk
from tkinter import ttk
import time
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MED

class AILifecyclePage:
    def _execute_ai_next(self):
        mid = self._ai_active_mid.get()
        if mid == 'N/A':
            return
        res = self.kernel.ai_lifecycle.execute_next_step(mid)
        self._notify('Phase Complete', f"Completed: {res.get('step', 'unknown')}", 'OK')
        self._update_ai_missions()