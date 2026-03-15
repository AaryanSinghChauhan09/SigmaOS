# Generated method: ShoppingWizardPage.__init__
import tkinter as tk
from tkinter import ttk
import random
import webbrowser
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED, FONT_TITLE

class ShoppingWizardPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, '🛒 BuyingHatke Wizard', 'Sovereign Price Intelligence & Enterprise Commerce Hub')
        self._build_ui()