# Generated method: SovereignPDFEditor._setup_styles
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import random

class SovereignPDFEditor:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('PDF.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('PDF.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[15, 8], font=('Inter', 9, 'bold'))
        style.map('PDF.TNotebook.Tab', background=[('selected', PAL['accent'])])