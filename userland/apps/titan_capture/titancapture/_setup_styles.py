# Generated method: TitanCapture._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import time
import random

class TitanCapture:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Titan.TCombobox', fieldbackground='#000', background='#000', foreground='white', bordercolor=PAL['border'])