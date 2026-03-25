import tkinter as tk
from .mixins import UIMixin
from .styles import PAL, FONT_SMALL, FONT_BOLD

class SigmaPage(tk.Frame, UIMixin):
    """Base class for all SigmaOS GUI pages."""
    def __init__(self, parent, gui, title, subtitle):
        super().__init__(parent, bg=PAL["bg"])
        self.gui = gui
        self.kernel = gui.kernel
        self.title = title
        self.subtitle = subtitle
        self._build_header()
        
    def _build_header(self):
        self._build_page_header(self, self.title, self.subtitle)

    def _notify(self, title, msg, level="INFO"):
        self.gui._notify(title, msg, level)

    def _morphic_island(self, msg, color=None, duration=5000):
        self.gui._morphic_island(msg, color, duration)
        
    def after(self, ms, func, *args):
        return self.gui.after(ms, func, *args)
