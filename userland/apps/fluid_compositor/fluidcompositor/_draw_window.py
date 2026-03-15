# Generated method: FluidCompositor._draw_window
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import sys
import os
from typing import Dict, Any, List, Optional
from sigma_core.ui.zbuffer_engine import ZBufferEngine
from sigma_core.ui.rendering_pipeline import RenderingPipeline
from sigma_core.ui.fluid_design import PALETTE as PAL, FluidTheme

class FluidCompositor:
    def _draw_window(self, win):
        x, y, w, h = (win['x'], win['y'], win['w'], win['h'])
        self.canvas.create_rectangle(x - 5, y - 5, x + w + 5, y + h + 5, fill='', outline=win['color'], width=1, tags='ui', dash=(4, 4))
        self.canvas.create_rectangle(x, y, x + w, y + h, fill='#121418', outline='#2A2D35', tags='ui')
        self.canvas.create_rectangle(x, y, x + w, y + 30, fill='#1A1C23', outline='#2A2D35', tags='ui')
        self.canvas.create_text(x + 10, y + 15, text=win['name'], fill='white', anchor='w', font=('Inter Bold', 9), tags='ui')
        self.canvas.create_oval(x + w - 50, y + 10, x + w - 40, y + 20, fill='#FF5F57', outline='', tags='ui')
        self.canvas.create_oval(x + w - 30, y + 10, x + w - 20, y + 20, fill='#FFBD2E', outline='', tags='ui')