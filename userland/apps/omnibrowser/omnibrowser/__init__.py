# Generated method: OmniBrowser.__init__
import tkinter as tk
from tkinter import ttk, messagebox, simpledialog
import random
import time
from typing import Any, List
from sigma_core.ui.fluid_design import ICONS

class OmniBrowser:
    def __init__(self, master=None):
        super().__init__(master)
        self.title('OmniBrowser Apex Pro - [Secure Sandbox]')
        self.geometry('1200x850')
        self.config(bg=PAL['bg'])
        self.tabs: List[str] = ['omni.sigma://home', 'sovereign.vault/auth', 'github.com/sigmaos']
        self.active_tab_idx = 0
        self.tab_container: Any = None
        self.tab_nb: Any = None
        self.addr_fr: Any = None
        self.url_entry: Any = None
        self.shield_btn: Any = None
        self.viewport: Any = None
        self.render_lbl: Any = None
        self.status: Any = None
        self._setup_styles()
        self._build_ui()