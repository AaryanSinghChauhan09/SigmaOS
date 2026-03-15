# Generated file: draw_square
import tkinter as tk
from tkinter import messagebox
import random
import time
import math

def draw_square(c, cx, cy, s, col, outline='#FFFFFF'):
    c.create_rectangle(cx - s, cy - s, cx + s, cy + s, fill=col, outline=outline, width=2)