# Generated method: PulsePlayer._filter_tracks
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict

class PulsePlayer:
    def _filter_tracks(self, event=None):
        q = self.srch.get().lower()
        self.queue_tree.delete(*self.queue_tree.get_children())
        for t in self.tracks:
            title = str(t.get('title', '')).lower()
            artist = str(t.get('artist', '')).lower()
            if q in title or q in artist:
                self.queue_tree.insert('', 'end', values=(t['title'], t['dur']))