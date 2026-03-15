"""
Auto-split from userland\apps\omni_purge.py — OmniPurge._analyze_space
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class OmniPurge:
    def _analyze_space(self):
        self.status.config(text='SCANNING NVME OMNI-BUS FOR SELECTED VECTORS...', bg=PAL['warning'], fg='black')
        gig_estimate = sum([random.uniform(0.5, 4.0) for v in self.categories.values() if v.get()])
        self.after(1500, lambda: self._complete_analysis(gig_estimate))
