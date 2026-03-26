# Generated method: ArcadePage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL

class ArcadePage:
    def __init__(self, parent, gui):
        is_child = gui.kernel.registry.get('guardian').is_child_mode()
        title = 'KIDDIE PLAYGROUND' if is_child else 'SOVEREIGN ARCADE'
        subtitle = 'Safe & Fun Games for Little Champions!' if is_child else 'Zero-Telemetry Clean-Room Game Engine (64+ Logic Modules)'
        super().__init__(parent, gui, title, subtitle)
        self.build()