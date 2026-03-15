"""
Auto-split from userland\apps\spot_it.py — draw_circle
"""

import tkinter as tk
from tkinter import messagebox
import random
import time
import math



def draw_circle(c, cx, cy, s, col, outline='#FFFFFF'):
    c.create_oval(cx - s, cy - s, cx + s, cy + s, fill=col, outline=outline, width=2)
