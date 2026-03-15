# Generated method: PulsePlayer._update_track_display
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict

class PulsePlayer:
    def _update_track_display(self):
        if not self.tracks:
            return
        t = self.tracks[self.current_idx]
        self.title_lbl.config(text=t['title'])
        self.artist_lbl.config(text=t['artist'])
        self.dur_lbl.config(text=t['dur'])
        self.title(f"PulsePlayer — {t['title']}")
        icons = ['💿', '🎵', '🎸', '🎹', '🎻']
        self.art_lbl.config(text=random.choice(icons))