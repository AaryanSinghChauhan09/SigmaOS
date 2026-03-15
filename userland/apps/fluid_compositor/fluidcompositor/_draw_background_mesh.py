# Generated method: FluidCompositor._draw_background_mesh
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
    def _draw_background_mesh(self):
        """USP: Low-latency procedural background grid."""
        for i in range(0, 1400, 50):
            self.canvas.create_line(i, 0, i, 900, fill='#1A1C20', tags='bg')
        for i in range(0, 900, 50):
            self.canvas.create_line(0, i, 1400, i, fill='#1A1C20', tags='bg')