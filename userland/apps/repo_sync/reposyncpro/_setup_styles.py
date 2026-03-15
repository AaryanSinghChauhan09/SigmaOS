# Generated method: RepoSyncPro._setup_styles
import tkinter as tk
from tkinter import ttk, messagebox
import subprocess
import threading
import os

class RepoSyncPro:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Git.TProgressbar', background=PAL['accent'], troughcolor=PAL['border'], borderwidth=0)