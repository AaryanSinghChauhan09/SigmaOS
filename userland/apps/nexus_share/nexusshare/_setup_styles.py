# Generated method: NexusShare._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading

class NexusShare:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Nexus.TProgressbar', background=PAL['accent'], troughcolor=PAL['sidebar'], borderwidth=0)