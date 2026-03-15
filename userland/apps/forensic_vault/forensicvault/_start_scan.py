# Generated method: ForensicVault._start_scan
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import os, hashlib, time, threading

class ForensicVault:
    def _start_scan(self):
        if self._running:
            return
        self._running = True
        self._log('COMMENCING DEEP FORENSIC SCAN...', 'ACTION')
        threading.Thread(target=self._scan_worker, daemon=True).start()