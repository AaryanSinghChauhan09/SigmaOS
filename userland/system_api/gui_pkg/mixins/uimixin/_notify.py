# Generated method: UIMixin._notify
import tkinter as tk
from tkinter import scrolledtext, messagebox
from .styles import PAL, FONT_MONO, FONT_SMALL, FONT_BOLD

class UIMixin:
    def _notify(self, title: str, msg: str, level: str='INFO'):
        """System notification proxy."""
        if hasattr(self, 'master') and hasattr(self.master, '_notify'):
            self.master._notify(title, msg, level)
        else:
            print(f'[{level}] {title}: {msg}')