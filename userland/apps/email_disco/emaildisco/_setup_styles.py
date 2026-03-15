# Generated method: EmailDisco._setup_styles
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random
import time

class EmailDisco:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Treeview', background=PAL['panel'], foreground=PAL['text'], fieldbackground=PAL['panel'], borderwidth=0, font=('Inter', 9))
        style.map('Treeview', background=[('selected', PAL['accent'])])