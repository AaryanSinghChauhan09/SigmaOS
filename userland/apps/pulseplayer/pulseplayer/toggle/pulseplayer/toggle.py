# Generated method: PulsePlayer.toggle
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict

class PulsePlayer:
    def toggle(self):
        self.playing = not self.playing
        self.play_btn.config(text='⏸' if self.playing else '▶')
        if self.playing:
            self._start_viz()