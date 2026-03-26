# Generated method: VanguardPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class VanguardPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, 'Vanguard Security Hub', 'Silo-Isolation & Zero-Persistence Engine')
        self.build()