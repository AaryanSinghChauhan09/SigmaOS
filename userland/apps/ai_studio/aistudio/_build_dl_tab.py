"""
Auto-split from userland\apps\ai_studio.py — AIStudio._build_dl_tab
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
    def _build_dl_tab(self, parent):
        self.tabs.add(parent, text=f" {ICONS.get('fabric', '🕸️')} NEURAL ")
        tk.Label(parent, text='DEEP LEARNING TOPOLOGY', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        archs = [('Transformer (LLM)', 'Attention Mechanisms. Mixed Precision FP8.'), ('Convolutional (CNN)', 'Vision Protocols. Tensor Cores Active.'), ('Generative Adversarial (GAN)', 'Zero-Sum Game theory matrix.')]
        for name, d in archs:
            lbl = tk.Label(parent, text=f'💠 {name}', font=('Inter', 10, 'bold'), fg=PAL['text'], bg=PAL['panel'], padx=15, pady=10, anchor='w')
            lbl.pack(fill='x', pady=4)
            tk.Label(lbl, text=d, fg=PAL['dim'], bg=PAL['panel'], font=('Inter', 8)).pack(side='right', padx=10)
