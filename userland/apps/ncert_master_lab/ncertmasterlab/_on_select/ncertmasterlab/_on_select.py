# Generated method: NCERTMasterLab._on_select
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import importlib, sys, os, traceback, json, time
from typing import Dict, Any, List, Optional

class NCERTMasterLab:
    def _on_select(self, _):
        sel = self._tree.selection()
        if not sel or sel[0] not in self._exp_map:
            return
        self._build_form(*self._exp_map[sel[0]])