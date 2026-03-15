"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._build_misc_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _build_misc_tab(self) -> None:
        tk.Label(self.tab_misc, text='Miscellaneous Utilities  (offline)', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        mf = tk.Frame(self.tab_misc, bg=PAL['panel'], padx=15, pady=15)
        mf.pack(fill='x', pady=5)
        tk.Button(mf, text='🔦  FLASHLIGHT (3 s)', bg=PAL['warning'], fg='black', font=('Inter', 9, 'bold'), command=self._flashlight).grid(row=0, column=0, padx=8, pady=6)
        tk.Button(mf, text='🧭  COMPASS', bg=PAL['accent_dim'], fg='black', font=('Inter', 9, 'bold'), command=self._compass).grid(row=0, column=1, padx=8, pady=6)
        tk.Button(mf, text='🎨  COLOR PICKER', bg=PAL['accent'], fg='black', font=('Inter', 9, 'bold'), command=self._pick_color).grid(row=0, column=2, padx=8, pady=6)
        tk.Button(mf, text='📅  DATE DIFF', bg=PAL['sidebar'], fg='white', font=('Inter', 9, 'bold'), command=self._date_diff).grid(row=0, column=3, padx=8, pady=6)
