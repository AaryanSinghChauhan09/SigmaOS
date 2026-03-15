# Generated method: MorphicDashboard._update_loop
import tkinter as tk
from tkinter import ttk, messagebox
import sys, os, time, random
from typing import Dict, Any, List, Optional

class MorphicDashboard:
    def _update_loop(self):
        if self.time_lbl:
            self.time_lbl.config(text=time.strftime('%H:%M:%S'))
        hal = self.hal
        if hal and hasattr(hal, 'get_hardware_state'):
            state = hal.get_hardware_state()
            if state and self.cpu_val:
                self.cpu_val.config(text=f"{state.get('cpu', 0)}%")
            if state and self.ram_val:
                self.ram_val.config(text=f"{state.get('ram', 0)}%")
        if self.edu and hasattr(self.edu, 'xp') and self.xp_val:
            self.xp_val.config(text=str(getattr(self.edu, 'xp', 0)))
        if self.ai and hasattr(self.ai, 'platforms') and self.ai_val:
            platforms = getattr(self.ai, 'platforms', [])
            nodes = len(platforms) if platforms else 0
            self.ai_val.config(text=f'{nodes} NODES')
        if self.cpu_icon:
            self.cpu_icon.config(text=SPINNERS['pulse'][self._anim_counters['cpu'] % len(SPINNERS['pulse'])])
            self._anim_counters['cpu'] += 1
        if self.ram_icon:
            self.ram_icon.config(text=SPINNERS['gear'][self._anim_counters['ram'] % len(SPINNERS['gear'])])
            self._anim_counters['ram'] += 1
        if self.xp_icon:
            self.xp_icon.config(text=SPINNERS['orbit'][self._anim_counters['xp'] % len(SPINNERS['orbit'])])
            self._anim_counters['xp'] += 1
        if self.ai_icon:
            self.ai_icon.config(text=SPINNERS['neural'][self._anim_counters['ai'] % len(SPINNERS['neural'])])
            self._anim_counters['ai'] += 1
        self.after(500, self._update_loop)