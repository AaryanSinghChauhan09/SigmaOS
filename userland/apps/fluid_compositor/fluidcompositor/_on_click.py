# Generated method: FluidCompositor._on_click
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
    def _on_click(self, event):
        clicked_win = None
        for win_id, win in reversed(list(self.windows.items())):
            if win['x'] < event.x < win['x'] + win['w'] and win['y'] < event.y < win['y'] + win['h']:
                clicked_win = win
                break
        if clicked_win is not None:
            max_z = max((w['z'] for w in self.windows.values()))
            clicked_win['z'] = max_z + 1