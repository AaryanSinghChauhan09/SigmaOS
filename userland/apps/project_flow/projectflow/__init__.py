# Generated method: ProjectFlow.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class ProjectFlow:
    def __init__(self):
        super().__init__()
        self.title('ProjectFlow Apex Pro v4.0')
        self.geometry('1300x900')
        self.configure(bg=PAL['bg'])
        self._setup_style()
        self._build_ui()
        self._set_status('TIMELINE SYNCED | NEURAL ESTIMATION ACTIVE', PAL['accent'])