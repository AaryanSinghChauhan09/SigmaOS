# Generated method: VisionExplorer.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import random
import math
from typing import Dict, Any, List, Optional

class VisionExplorer:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title('Sovereign Vision Explorer — System Shard Map')
        self.geometry('1100x750')
        self.configure(bg=PAL['bg'])
        self.header = tk.Frame(self)
        self.workspace = tk.Frame(self)
        self.index_fr = tk.Frame(self)
        self.canvas_fr = tk.Frame(self)
        self.canvas = tk.Canvas(self)
        self.active_shard = 'KERNEL'
        self._build_ui()
        self._animate_nodes()