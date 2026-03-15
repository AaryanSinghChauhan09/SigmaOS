"""
Auto-split from userland\apps\codeforge.py — SovereignCodeForge._setup_styles
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional



class SovereignCodeForge:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        for name, cfg in {'Treeview': {'background': PAL['sidebar'], 'foreground': PAL['text'], 'fieldbackground': PAL['sidebar'], 'borderwidth': 0, 'font': ('Segoe UI', 9)}, 'Treeview.Heading': {'background': PAL['gutter'], 'foreground': PAL['dim'], 'font': ('Segoe UI', 8, 'bold')}}.items():
            style.configure(name, **cfg)
        style.map('Treeview', background=[('selected', PAL['accent'])])
