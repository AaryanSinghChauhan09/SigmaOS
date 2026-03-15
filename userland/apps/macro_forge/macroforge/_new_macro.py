"""
Auto-split from userland\apps\macro_forge.py — MacroForge._new_macro
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random



class MacroForge:
    def _new_macro(self):
        self.tree.delete(*self.tree.get_children())
        self.status.config(text='NEW SEQUENCE INITIATED. AWAITING TRIGGERS.', bg=PAL['panel'], fg='white')
