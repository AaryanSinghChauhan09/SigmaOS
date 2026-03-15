# Generated method: FluidCompositor._spawn_demo_windows
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
    def _spawn_demo_windows(self):
        names = ['AETHER_NAVIGATOR', 'NEURAL_TERMINAL', 'COGNITIVE_STUDIO']
        colors = ['#00D4FF', '#7000FF', '#00FF70']
        for i, name in enumerate(names):
            win_id = f'win_{i}'
            self.windows[win_id] = {'name': name, 'x': 100.0 + i * 150, 'y': 100.0 + i * 100, 'w': 400, 'h': 250, 'color': colors[i], 'z': i, 'vx': (random.random() - 0.5) * 2, 'vy': (random.random() - 0.5) * 2}