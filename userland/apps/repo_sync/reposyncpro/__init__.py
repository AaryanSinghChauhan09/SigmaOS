# Generated method: RepoSyncPro.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import subprocess
import threading
import os

class RepoSyncPro:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title('Sovereign Repo Sync Pro')
        self.geometry('850x650')
        self.configure(bg=PAL['bg'])
        self.repo_dir = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
        self._setup_styles()
        self._build_ui()