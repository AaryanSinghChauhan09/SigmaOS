# Generated method: SystemProfiler.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import platform
import random

class SystemProfiler:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title('Sovereign Sentinel APEX')
        self.geometry('900x650')
        self.configure(bg=PAL['bg'])
        self.cpu_usage = 0
        self.ram_usage = 0
        self.active_threads = 0
        self._setup_styles()
        self._build_ui()
        self._update_telemetry()