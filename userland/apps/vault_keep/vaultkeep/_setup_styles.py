# Generated method: VaultKeep._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import random

class VaultKeep:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Vault.Treeview', background=PAL['sidebar'], fieldbackground=PAL['sidebar'], foreground=PAL['text'], borderwidth=0, font=('Inter', 10), rowheight=35)
        style.configure('Vault.Treeview.Heading', background=PAL['panel'], foreground=PAL['dim'], font=('Inter', 9, 'bold'), borderwidth=0)
        style.map('Vault.Treeview', background=[('selected', PAL['accent_dim'])])