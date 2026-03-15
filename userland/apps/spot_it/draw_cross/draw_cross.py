# Generated file: draw_cross
import tkinter as tk
from tkinter import messagebox
import random
import time
import math

def draw_cross(c, cx, cy, s, col, outline='#FFFFFF'):
    t = s // 3
    c.create_rectangle(cx - t, cy - s, cx + t, cy + s, fill=col, outline=outline, width=2)
    c.create_rectangle(cx - s, cy - t, cx + s, cy + t, fill=col, outline=outline, width=2)