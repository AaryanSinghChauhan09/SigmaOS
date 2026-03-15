# Generated method: PulsePlayer._toggle_spatial
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict

class PulsePlayer:
    def _toggle_spatial(self):
        msg = 'SPATIAL AUDIO: ENGAGED | Virtual 7.1 Orbit active' if self._spatial_audio.get() else 'SPATIAL AUDIO: DISENGAGED'
        self._notify_status(msg)