# Generated method: OmniConverter.__init__
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
from typing import List, Dict

class OmniConverter:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title('Sovereign OmniConverter Apex Pro')
        self.geometry('900x700')
        self.configure(bg=PAL['bg'])
        self.source_file = None
        self._build_ui()