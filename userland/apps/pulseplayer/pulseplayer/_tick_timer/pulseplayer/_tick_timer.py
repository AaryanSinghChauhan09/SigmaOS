# Generated method: PulsePlayer._tick_timer
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict

class PulsePlayer:
    def _tick_timer(self):
        if self.playing:
            t = self.tracks[self.current_idx]
            dur_s = float(t.get('dur_s', 300))
            if dur_s > 0:
                self._progress = min(self._progress + 1.0, dur_s)
                pct = self._progress / dur_s * 100
                self.prog_var.set(pct)
                elapsed = int(self._progress)
                self.time_lbl.config(text=f'{elapsed // 60}:{elapsed % 60:02d}')
                if self._progress >= dur_s:
                    self._next()
        if self.playing:
            seq = SPINNERS.get('signal', ['📶', '🛜', '🌐'])
            icon = seq[self._anim_idx % len(seq)]
            self._set_status(f'{icon} BIT-PERFECT MASTER ACTIVE | NEURAL UPSAMPLING: 384KHz')
            self._anim_idx += 1
        self.after(1000, self._tick_timer)