# Generated file: draw_hexagon
import tkinter as tk
from tkinter import messagebox
import random
import time
import math

def draw_hexagon(c, cx, cy, s, col, outline='#FFFFFF'):
    pts = []
    for i in range(6):
        a = math.pi * i / 3
        pts.extend([cx + s * math.cos(a), cy + s * math.sin(a)])
    c.create_polygon(pts, fill=col, outline=outline, width=2)