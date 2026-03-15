# Generated method: MorphicDashboard._switch_vibe
import tkinter as tk
from tkinter import ttk, messagebox
import sys, os, time, random
from typing import Dict, Any, List, Optional

class MorphicDashboard:
    def _switch_vibe(self, vibe):
        if self.kernel and hasattr(self.kernel, 'bus'):
            self.kernel.bus.emit('governor.vibe_switch', {'vibe': vibe})
        messagebox.showinfo('Vibe Switch', f'System Atmosphere updated to: {vibe}')