# Generated method: FluidCompositor._render_loop
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
    def _render_loop(self):
        self.canvas.delete('ui')
        for win in self.windows.values():
            win['x'] += win['vx']
            win['y'] += win['vy']
            if win['x'] < 0 or win['x'] + win['w'] > 1400:
                win['vx'] *= -1
            if win['y'] < 0 or win['y'] + win['h'] > 900:
                win['vy'] *= -1
        sorted_wins = sorted(self.windows.values(), key=lambda x: x['z'])
        for win in sorted_wins:
            self._draw_window(win)
        self.after(20, self._render_loop)