# Generated method: SigmaStopwatch._sw_lap
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

class SigmaStopwatch:
    def _sw_lap(self):
        elapsed = self._sw_elapsed + (time.time() - self._sw_start if self._sw_running else 0)
        h = int(elapsed // 3600)
        m = int(elapsed % 3600 // 60)
        s = int(elapsed % 60)
        cs = int(elapsed % 1 * 100)
        lap = f'Lap {len(self._laps) + 1:>3}: {h:02}:{m:02}:{s:02}.{cs:02}\n'
        self._laps.append(lap)
        self._lap_box.insert('end', lap)
        self._lap_box.see('end')