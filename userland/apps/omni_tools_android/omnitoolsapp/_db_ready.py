"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._db_ready
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _db_ready(self) -> None:
        self.status.config(text='Offline DB up-to-date  |  No network access required', bg=PAL['success'], fg='black')
