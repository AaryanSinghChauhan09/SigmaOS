# Generated method: EchoCast.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import threading
import time
import random

class EchoCast:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title('Sovereign Echo Cast')
        self.geometry('900x600')
        self.configure(bg=PAL['bg'])
        self.scanning = False
        self._setup_styles()
        self._build_ui()