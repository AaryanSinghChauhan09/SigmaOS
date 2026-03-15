# Generated method: EchoCast._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import threading
import time
import random

class EchoCast:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Cast.TProgressbar', background=PAL['accent'], troughcolor=PAL['sidebar'], borderwidth=0)