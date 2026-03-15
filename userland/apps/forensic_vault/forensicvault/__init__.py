# Generated method: ForensicVault.__init__
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import os, hashlib, time, threading

class ForensicVault:
    def __init__(self):
        super().__init__()
        self.title('SigmaOS Forensic Vault v1.0')
        self.geometry('1100x800')
        self.configure(bg=PAL['bg'])
        self.monitored_paths = [os.getcwd()]
        self.file_hashes = {}
        self._running = False
        self._build_ui()
        self._log('FORENSIC SENTINEL ONLINE | WAITING FOR INTEGRITY SCAN')