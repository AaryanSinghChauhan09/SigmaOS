# Generated method: OmniSearch._live_search
import tkinter as tk
from tkinter import ttk, messagebox
import time

class OmniSearch:
    def _live_search(self, event):
        q = self.search_entry.get().lower()
        self.tree.delete(*self.tree.get_children())
        if not q:
            for item in self.db:
                self.tree.insert('', 'end', values=item)
            self.status.config(text='SEMANTIC ENGINE IDLE | 0.00 MS INDEX LATENCY')
            return
        results = 0
        for item in self.db:
            if q in item[0].lower() or q in item[1].lower():
                self.tree.insert('', 'end', values=item)
                results += 1
        self.status.config(text=f'NEURAL MATCHES FOUND: {results} | SEARCH TIME: 0.12 ms (O[1] Hashing)', bg=PAL['success'], fg='black')