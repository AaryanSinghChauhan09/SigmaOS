# Generated method: SigmaPage.__init__
import tkinter as tk
from .mixins import UIMixin
from .styles import PAL, FONT_SMALL, FONT_BOLD

class SigmaPage:
    def __init__(self, parent, gui, title, subtitle):
        super().__init__(parent, bg=PAL['bg'])
        self.gui = gui
        self.controller = gui
        self.kernel = gui.kernel
        self.title = title
        self.subtitle = subtitle
        self._build_header()