# Generated method: SovereignCodeForge._setup_tags
import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional

class SovereignCodeForge:
    def _setup_tags(self):
        for name, (_, color) in KEYWORDS.items():
            self.txt.tag_config(name, foreground=color)