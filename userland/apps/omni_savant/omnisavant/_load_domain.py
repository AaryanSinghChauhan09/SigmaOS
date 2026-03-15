"""
Auto-split from userland\apps\omni_savant.py — OmniSavant._load_domain
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class OmniSavant:
    def _load_domain(self, domain):
        self.dom_title.config(text=f'DOMAIN MATRIX: {domain.upper()}')
        self.tree.delete(*self.tree.get_children())
        for arch, theorem, impl in self.knowledge_base[domain]:
            self.tree.insert('', 'end', values=(arch, theorem, impl))
        self.status.config(text=f'VECTORS LOADED FOR: {domain} | MEMORY ALLOCATION COMPLETED', bg=PAL['sidebar'], fg='white')
