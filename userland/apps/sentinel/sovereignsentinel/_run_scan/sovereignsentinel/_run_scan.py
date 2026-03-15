# Generated method: SovereignSentinel._run_scan
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess

class SovereignSentinel:
    def _run_scan(self):
        if self._scanning:
            return
        self._scanning = True
        self.scan_log.delete('1.0', 'end')
        self.scan_log.insert('end', f'[GUARDIAN] Initiating: {self._scan_type.get()}\n', None)
        self.scan_prog['value'] = 0
        threading.Thread(target=self._scan_worker, daemon=True).start()