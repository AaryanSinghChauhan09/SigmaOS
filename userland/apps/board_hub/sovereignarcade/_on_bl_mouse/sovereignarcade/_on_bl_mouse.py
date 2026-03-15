# Generated method: SovereignArcade._on_bl_mouse
import tkinter as tk
from tkinter import ttk, messagebox
import random, time, os, sys
from typing import Dict, Any, List, Optional

class SovereignArcade:
    def _on_bl_mouse(self, event):
        if self.bl_active:
            x = max(40, min(360, event.x))
            self.bl_canv.coords(self.paddle, x - 40, 480, x + 40, 490)