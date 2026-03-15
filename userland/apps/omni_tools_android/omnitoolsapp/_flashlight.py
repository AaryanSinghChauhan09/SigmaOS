"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._flashlight
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _flashlight(self) -> None:
        orig = self.cget('bg')
        self.configure(bg='#FFFFFF')
        self.after(3000, self.configure, {'bg': orig})
