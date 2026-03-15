# Generated method: ToolsFinder._setup_styles
import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import os
import time

class ToolsFinder:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Treeview', background=PAL['sidebar'], foreground=PAL['text'], fieldbackground=PAL['sidebar'], borderwidth=0, font=('Inter', 9))
        style.map('Treeview', background=[('selected', PAL['accent'])])