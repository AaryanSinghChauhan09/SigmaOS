"""
Auto-split from userland\apps\advocate_command_center.py — AdvocateCommandCenter._setup_styles
"""

import tkinter as tk
from tkinter import ttk, messagebox
from typing import Dict, Any, List, Optional, Union, Callable
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT



class AdvocateCommandCenter:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Custom.Treeview', background=PAL['surface'], foreground=PAL['text_primary'], fieldbackground=PAL['surface'], borderwidth=0, font=FONT['body'])
        style.configure('Custom.Treeview.Heading', background=PAL['surface_variant'], foreground=PAL['secondary'], font=FONT['body_bold'])
        style.map('Custom.Treeview', background=[('selected', PAL['secondary'])], foreground=[('selected', PAL['background'])])
