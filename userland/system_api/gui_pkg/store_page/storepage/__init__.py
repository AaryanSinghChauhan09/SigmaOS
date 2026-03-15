# Generated method: StorePage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class StorePage:
    def __init__(self, parent, gui):
        is_child = gui._is_child_mode()
        title = 'Kiddie Toy Shop' if is_child else 'Sovereign App Store'
        subtitle = 'Fun & Safe Toys for Everyone!' if is_child else 'Verified Zero-Trust Applications & Games'
        super().__init__(parent, gui, title, subtitle)
        self._build_ui()