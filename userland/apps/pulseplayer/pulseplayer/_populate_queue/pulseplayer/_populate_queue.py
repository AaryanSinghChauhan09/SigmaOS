# Generated method: PulsePlayer._populate_queue
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import time, random, threading, os
from typing import Any, List, Dict

class PulsePlayer:
    def _populate_queue(self):
        self.queue_tree.delete(*self.queue_tree.get_children())
        for t in self.tracks:
            self.queue_tree.insert('', 'end', values=(t['title'], t['dur']))
        children = self.queue_tree.get_children()
        if children and self.current_idx < len(children):
            self.queue_tree.selection_set(children[self.current_idx])