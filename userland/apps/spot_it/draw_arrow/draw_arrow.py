# Generated file: draw_arrow
import tkinter as tk
from tkinter import messagebox
import random
import time
import math

def draw_arrow(c, cx, cy, s, col, outline='#FFFFFF'):
    pts = [cx, cy - s, cx + s, cy, cx + s // 2, cy, cx + s // 2, cy + s, cx - s // 2, cy + s, cx - s // 2, cy, cx - s, cy]
    c.create_polygon(pts, fill=col, outline=outline, width=2)