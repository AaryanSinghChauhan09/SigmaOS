# Generated file: draw_diamond
import tkinter as tk
from tkinter import messagebox
import random
import time
import math

def draw_diamond(c, cx, cy, s, col, outline='#FFFFFF'):
    pts = [cx, cy - s, cx + s, cy, cx, cy + s, cx - s, cy]
    c.create_polygon(pts, fill=col, outline=outline, width=2)