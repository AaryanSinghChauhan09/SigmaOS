# Generated method: EmailDisco.__init__
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random
import time

class EmailDisco:
    def __init__(self):
        super().__init__()
        self.title('Sovereign Email Pro Apex Pro')
        self.geometry('1200x850')
        self.configure(bg=PAL['bg'])
        self._setup_styles()
        self._build_ui()
        self._sync_threads()