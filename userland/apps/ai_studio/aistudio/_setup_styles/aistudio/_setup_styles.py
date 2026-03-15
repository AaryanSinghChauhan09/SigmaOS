# Generated method: AIStudio._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading
import sys
import os
from typing import Dict, Any, List, Optional

class AIStudio:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Studio.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('Studio.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[15, 8], font=('Inter', 9, 'bold'))
        style.map('Studio.TNotebook.Tab', background=[('selected', PAL['accent'])])
        style.configure('Studio.Horizontal.TProgressbar', background=PAL['accent'], troughcolor=PAL['sidebar'], borderwidth=0)