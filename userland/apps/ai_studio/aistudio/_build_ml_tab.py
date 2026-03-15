"""
Auto-split from userland\apps\ai_studio.py — AIStudio._build_ml_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading
import sys
import os
from typing import Dict, Any, List, Optional



class AIStudio:
    def _build_ml_tab(self, parent):
        self.tabs.add(parent, text=f" {ICONS.get('ml_engine', '🧪')} ALGORITHMS ")
        tk.Label(parent, text='CLASSIFICATION / REGRESSION', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        algos = ['XGBoost', 'Random Forest', 'SVM (RBF Kernel)', 'K-Means++']
        for act in algos:
            cb = tk.Checkbutton(parent, text=act, bg=PAL['bg'], fg=PAL['text'], selectcolor=PAL['panel'], font=('Inter', 10), activebackground=PAL['bg'])
            cb.pack(anchor='w', pady=6)
            cb.select()
