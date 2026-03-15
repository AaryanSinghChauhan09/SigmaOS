"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._gen_password
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _gen_password(self) -> None:
        length = int(self.pwd_len.get())
        alphabet = string.ascii_letters + string.digits + '!@#$%^&*()-_=+'
        pwd = ''.join((secrets.choice(alphabet) for _ in range(length)))
        self.pwd_result.delete(0, tk.END)
        self.pwd_result.insert(0, pwd)
        self.clipboard_clear()
        self.clipboard_append(pwd)
        self.status.config(text='Password generated & copied to clipboard.', bg=PAL['success'], fg='black')
