# Generated method: BrowserPage.__init__
import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_MED, FONT_SMALL

class BrowserPage:
    def __init__(self, parent, gui):
        guardian = gui.kernel.registry.get('guardian')
        is_child = guardian and guardian.is_child_mode()
        title = 'Kiddy Browser' if is_child else 'Sovereign Browser Pro'
        subtitle = 'Fun & Safe Web for Little Heroes!' if is_child else 'Absorption of Chrome/Arc/Safari — Zero-Trust Rendering'
        SigmaPage.__init__(self, parent, gui, title, subtitle)
        self.browser = self.kernel.registry.get('browser')
        self._build_ui()