"""
Auto-split from userland\apps\pulseplayer.py — PulsePlayer._toggle_sov
"""

import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict



class PulsePlayer:
    def _toggle_sov(self):
        msg = 'SOVEREIGN MODE: ACTIVE | ZERO-TELEMETRY PIPELINE ENGAGED' if self._sovereign_mode.get() else 'SOVEREIGN MODE: DISABLED | Standard Playback Active'
        self._set_status(msg)
        self._notify_status('DRC: HARD-BYPASS' if self._sovereign_mode.get() else 'DRC: NORMAL')
