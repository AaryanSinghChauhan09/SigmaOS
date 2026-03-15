"""
Auto-split from userland\apps\spot_it.py — draw_star
"""

import tkinter as tk
from tkinter import messagebox
import random
import time
import math



def draw_star(c, cx, cy, s, col, outline='#FFFFFF'):
    pts = []
    for i in range(10):
        angle = math.pi * i / 5 - math.pi / 2
        r = s if i % 2 == 0 else s * 0.45
        pts.extend([cx + r * math.cos(angle), cy + r * math.sin(angle)])
    c.create_polygon(pts, fill=col, outline=outline, width=2)
