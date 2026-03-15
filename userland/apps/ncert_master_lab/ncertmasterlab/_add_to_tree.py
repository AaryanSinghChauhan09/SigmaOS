"""
Auto-split from userland\apps\ncert_master_lab.py — NCERTMasterLab._add_to_tree
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import importlib, sys, os, traceback, json, time
from typing import Dict, Any, List, Optional



class NCERTMasterLab:
    def _add_to_tree(self, label, registry, color):
        root = self._tree.insert('', 'end', text=label, open=True)
        for cls_label, cls_obj in registry.items():
            cls_node = self._tree.insert(root, 'end', text=cls_label)
            for exp_display, data in cls_obj.EXP_DATA.items():
                node = self._tree.insert(cls_node, 'end', text=f'• {exp_display}')
                self._exp_map[node] = (cls_obj, exp_display, data, color)
