# Generated method: RepoSyncPro._audit_repo
import tkinter as tk
from tkinter import ttk, messagebox
import subprocess
import threading
import os

class RepoSyncPro:
    def _audit_repo(self):
        self._log('RUNNING REPOSITORY AUDIT (git status)...')
        res = self._run_git_cmd(['git', 'status'])
        self._log(res)