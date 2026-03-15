"""
Auto-split from userland\apps\markdown_viewer.py — launch
"""

import tkinter as tk
from tkinter import ttk, filedialog, scrolledtext
import re, os



def launch(kernel=None):
    MarkdownViewer(kernel).mainloop()
