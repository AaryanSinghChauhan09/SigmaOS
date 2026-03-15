"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._calc_emi
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _calc_emi(self) -> None:
        try:
            P = float(self.loan_principal_entry.get())
            r = float(self.loan_rate_entry.get()) / 100 / 12
            n = int(self.loan_years_entry.get()) * 12
            emi = P * r * (1 + r) ** n / ((1 + r) ** n - 1)
            self.emi_result.config(text=f'EMI: ${fmt(emi)}', fg=PAL['success'])
        except Exception:
            self.emi_result.config(text='Error: check inputs', fg=PAL['danger'])
