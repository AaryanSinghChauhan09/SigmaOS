"""
Auto-split from userland\apps\ncert_master_lab.py — NCERTMasterLab._load_backends
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import importlib, sys, os, traceback, json, time
from typing import Dict, Any, List, Optional



class NCERTMasterLab:
    def _load_backends(self):
        backend_info = [('ncert_physics_lab', 'PHYSICS_REGISTRY', PAL['ph'], f"{ICONS.get('ncert', '⚛')} Physics (6-12)"), ('ncert_chemistry_lab', 'CHEMISTRY_REGISTRY', PAL['ch'], f"{ICONS.get('genai_lab', '🧪')} Chemistry (6-12)"), ('ncert_biology_lab', 'BIOLOGY_REGISTRY', PAL['bi'], f"{ICONS.get('ml_engine', '🧬')} Biology (6-12)"), ('ncert_maths_lab', 'MATHS_REGISTRY', PAL['ma'], f"{ICONS.get('calculator', '📐')} Mathematics (1-12)")]
        for mod_name, reg_name, color, label in backend_info:
            try:
                mod = importlib.import_module(mod_name)
                registry = getattr(mod, reg_name)
                self._add_to_tree(label, registry, color)
            except:
                pass
