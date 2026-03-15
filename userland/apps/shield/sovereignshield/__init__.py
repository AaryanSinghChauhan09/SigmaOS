# Generated method: SovereignShield.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
from typing import Any
from sigma_core.ui.fluid_design import ICONS, SPINNERS

class SovereignShield:
    def __init__(self, master=None):
        super().__init__(master)
        self.title('Sovereign Shield SECURITY CENTER')
        self.geometry('700x500')
        self.config(bg='#0D0D15')
        self.header: Any = None
        self.scan_fr: Any = None
        self.status_lbl: Any = None
        self.prog: Any = None
        self.rules_fr: Any = None
        self._build_ui()