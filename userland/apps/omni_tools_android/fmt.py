"""
Auto-split from userland\apps\omni_tools_android.py — fmt
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



def fmt(n: float, d: int=4) -> str:
    return f'{n:.{d}g}' if isinstance(n, (int, float)) else str(n)
