# Generated method: PulsePlayer._start_viz
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict

class PulsePlayer:
    def _start_viz(self):
        self._draw_viz()
        if self.playing:
            self.after(80, self._start_viz)