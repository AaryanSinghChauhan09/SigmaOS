# Generated method: SovereignStrategist._setup_styles
import tkinter as tk
from tkinter import messagebox, ttk
import random
import time
from typing import Dict, Any, List, Optional, Tuple, cast
import os
import sys

class SovereignStrategist:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('TProgressbar', thickness=10, troughcolor=PAL['sq_dark'], background=PAL['accent'])