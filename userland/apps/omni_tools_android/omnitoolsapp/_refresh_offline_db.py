"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._refresh_offline_db
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _refresh_offline_db(self) -> None:
        self.status.config(text='Refreshing offline caches…', bg=PAL['warning'], fg='black')
        self.after(1500, self._db_ready)
