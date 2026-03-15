# Generated method: AuraPaint._set_status
import tkinter as tk
from tkinter import colorchooser, messagebox, ttk, simpledialog, filedialog
import random
import os
from typing import Any

class AuraPaint:
    def _set_status(self, msg):
        self.status.config(text=msg.upper())