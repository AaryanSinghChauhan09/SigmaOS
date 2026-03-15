# Generated method: SovereignStrategist._update_status
import tkinter as tk
from tkinter import messagebox, ttk
import random
import time
from typing import Dict, Any, List, Optional, Tuple, cast
import os
import sys

class SovereignStrategist:
    def _update_status(self, msg, color=None):
        if not color:
            color = PAL['accent']
        self.status.config(text=msg.upper(), bg=color)