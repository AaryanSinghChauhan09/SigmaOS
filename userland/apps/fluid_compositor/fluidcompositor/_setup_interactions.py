# Generated method: FluidCompositor._setup_interactions
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
    def _setup_interactions(self):
        self.canvas.bind('<Button-1>', self._on_click)