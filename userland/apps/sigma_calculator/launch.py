"""
Auto-split from userland\apps\sigma_calculator.py — launch
"""

import tkinter as tk
from tkinter import ttk
import math, cmath, re, json, os
from typing import Any, List



def launch(kernel=None):
    SigmaCalculator(kernel).mainloop()
