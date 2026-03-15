# Generated method: TitanCapture._update
import tkinter as tk
from tkinter import ttk, messagebox
import time
import random

class TitanCapture:
    def _update(self):
        if self._recording:
            elapsed = int(time.time() - self._start_time)
            m, s = divmod(elapsed, 60)
            h, m = divmod(m, 60)
            self.timer.config(text=f'{h:02d}:{m:02d}:{s:02d}')
            self.after(1000, self._update)