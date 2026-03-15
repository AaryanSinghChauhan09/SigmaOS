# Generated method: SovereignArcade._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import random, time, os, sys
from typing import Dict, Any, List, Optional

class SovereignArcade:
    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use('clam')
        s.configure('TNotebook', background=PAL['bg'], borderwidth=0)
        s.configure('TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[20, 10])
        s.map('TNotebook.Tab', background=[('selected', PAL['accent'])])