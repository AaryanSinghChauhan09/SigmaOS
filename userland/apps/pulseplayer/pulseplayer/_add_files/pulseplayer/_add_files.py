# Generated method: PulsePlayer._add_files
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict

class PulsePlayer:
    def _add_files(self):
        files = filedialog.askopenfilenames(title='Add to Queue', filetypes=[('Audio Files', '*.mp3 *.wav *.flac *.ogg *.m4a'), ('All', '*.*')])
        for f in files:
            name = os.path.basename(f)
            self.tracks.append({'title': name, 'artist': 'Local Library', 'dur': '—', 'dur_s': 0})
        self._populate_queue()
        self._notify_status(f'Added {len(files)} files to queue.')