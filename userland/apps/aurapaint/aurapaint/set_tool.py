"""
Auto-split from userland\apps\aurapaint.py — AuraPaint.set_tool
"""

import tkinter as tk
from tkinter import colorchooser, messagebox, ttk, simpledialog, filedialog
import random
import os
from typing import Any



class AuraPaint:
    def set_tool(self, tool):
        self.tool = tool
        self._set_status(f'TOOL: {tool}')
