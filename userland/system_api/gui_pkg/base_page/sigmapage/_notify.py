# Generated method: SigmaPage._notify
import tkinter as tk
from .mixins import UIMixin
from .styles import PAL, FONT_SMALL, FONT_BOLD

class SigmaPage:
    def _notify(self, title, msg, level='INFO'):
        self.gui._notify(title, msg, level)