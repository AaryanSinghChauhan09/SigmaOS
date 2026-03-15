# Generated method: RepoSyncPro._run_git_cmd
import tkinter as tk
from tkinter import ttk, messagebox
import subprocess
import threading
import os

class RepoSyncPro:
    def _run_git_cmd(self, args):
        try:
            result = subprocess.run(args, cwd=self.repo_dir, capture_output=True, text=True, check=True)
            return result.stdout
        except subprocess.CalledProcessError as e:
            return e.stderr