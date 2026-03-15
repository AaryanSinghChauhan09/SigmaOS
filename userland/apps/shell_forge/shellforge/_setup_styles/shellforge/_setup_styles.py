# Generated method: ShellForge._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class ShellForge:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Shell.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('Shell.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[15, 8], font=('Inter', 9, 'bold'))
        style.map('Shell.TNotebook.Tab', background=[('selected', PAL['accent_dim'])])