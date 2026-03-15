"""
Auto-split from userland\apps\omni_etl_forge.py — OmniETLForge._add_node
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading



class OmniETLForge:
    def _add_node(self, name, ntype):
        self.canvas.delete('all')
        col = PAL['node_in'] if ntype == 'IN' else PAL['node_out']
        x = random.randint(50, 200) if ntype == 'IN' else random.randint(400, 550)
        y = random.randint(50, 400)
        self.nodes.append({'name': name, 'type': ntype, 'x': x, 'y': y, 'color': col})
        self._redraw_canvas()
