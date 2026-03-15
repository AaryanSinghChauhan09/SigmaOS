# Generated method: OmniToolsApp._pom_tick
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _pom_tick(self, secs: int, w: int, b: int, working: bool) -> None:
        if secs <= 0:
            if working:
                self.timer_label.config(text=f'🔴 Break: {b} min', fg=PAL['danger'])
                self.after(1000, self._pom_tick, b * 60, w, b, False)
            else:
                self.timer_label.config(text='✅ Session complete!', fg=PAL['success'])
            return
        m, s = divmod(secs, 60)
        col = PAL['success'] if working else PAL['danger']
        self.timer_label.config(text=f"{('🟢' if working else '🔴')} {m:02d}:{s:02d}", fg=col)
        self.after(1000, self._pom_tick, secs - 1, w, b, working)