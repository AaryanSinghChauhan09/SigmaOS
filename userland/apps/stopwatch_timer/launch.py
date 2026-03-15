"""
Auto-split from userland\apps\stopwatch_timer.py — launch
"""

import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any



def launch(kernel=None):
    SigmaStopwatch(kernel).mainloop()
