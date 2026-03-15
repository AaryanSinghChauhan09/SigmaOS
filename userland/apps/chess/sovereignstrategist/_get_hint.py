"""
Auto-split from userland\apps\chess.py — SovereignStrategist._get_hint
"""

import tkinter as tk
from tkinter import messagebox, ttk
import random
import time
from typing import Dict, Any, List, Optional, Tuple, cast
import os
import sys



class SovereignStrategist:
    def _get_hint(self):
        self._update_status('ANALYZING...', PAL['primary'])
        self.after(500, self._show_hint)
