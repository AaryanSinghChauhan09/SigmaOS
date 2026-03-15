# Generated method: OmniLensPro._finish_scan
import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import time
import threading

class OmniLensPro:
    def _finish_scan(self):
        self.scanning = False
        mocks = ["[TEXT] -> 'Sovereign Architecture Protocol v5'", "[TEXT] -> 'SigmaOS Terminal Keys'", "[LINK] -> 'https://sigma.local/secure'", "[OBJECT] -> Class: 'Quantum Motherboard' Conf: 98.2%", "[ENTITY] -> 'Encrypted Barcode Fragment'"]
        self.after(500, lambda: self._log('>>> EXTRACTION COMPLETE:'))
        for i, m in enumerate(mocks):
            self.after(1000 + i * 300, lambda msg=m: self._log(f'    {msg}'))
        self.after(3000, lambda: self.status.config(text='NEURAL PARSE COMPLETE | ZERO DATA EXFILTRATION', bg=PAL['success'], fg='black'))