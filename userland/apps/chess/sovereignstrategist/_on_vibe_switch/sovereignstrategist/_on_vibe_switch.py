# Generated method: SovereignStrategist._on_vibe_switch
import tkinter as tk
from tkinter import messagebox, ttk
import random
import time
from typing import Dict, Any, List, Optional, Tuple, cast
import os
import sys

class SovereignStrategist:
    def _on_vibe_switch(self, payload):
        if FLUID_PAL:
            PAL['bg'] = FLUID_PAL['background']
            PAL['accent'] = FLUID_PAL['primary']
            self.configure(bg=PAL['bg'])
            self._draw_board()