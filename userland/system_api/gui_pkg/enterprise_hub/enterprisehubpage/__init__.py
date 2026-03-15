# Generated method: EnterpriseHubPage.__init__
import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL

class EnterpriseHubPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, 'Enterprise Hub', 'Omni-Sovereign Business Intelligence & Process Automation')
        self.build()