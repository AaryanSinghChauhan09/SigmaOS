# Generated method: SigmaPage.after
import tkinter as tk
from .mixins import UIMixin
from .styles import PAL, FONT_SMALL, FONT_BOLD

class SigmaPage:
    def after(self, ms, func, *args):
        return self.gui.after(ms, func, *args)