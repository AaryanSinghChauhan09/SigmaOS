# Generated method: WelcomeAssistant._setup_styles
import tkinter as tk
from tkinter import ttk
import time
from typing import Any, List, Dict

class WelcomeAssistant:
    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use('clam')
        s.configure('Welcome.TProgressbar', thickness=8, troughcolor='#1A1A24', background=PAL['accent'])