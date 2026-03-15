# Generated method: VisionExplorer._animate_nodes
import tkinter as tk
from tkinter import ttk, messagebox
import random
import math
from typing import Dict, Any, List, Optional

class VisionExplorer:
    def _animate_nodes(self):
        cv = self.canvas
        for i in range(6):
            cv.move(f'pulse_{i}', random.choice([-1, 1]), random.choice([-1, 1]))
        self.after(500, self._animate_nodes)