# Generated file: draw_triangle
import tkinter as tk
from tkinter import messagebox
import random
import time
import math

def draw_triangle(c, cx, cy, s, col, outline='#FFFFFF'):
    pts = [cx, cy - s, cx + s, cy + s, cx - s, cy + s]
    c.create_polygon(pts, fill=col, outline=outline, width=2)