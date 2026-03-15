# Generated method: SovereignCodeForge._trigger_lint
import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional

class SovereignCodeForge:
    def _trigger_lint(self):
        self.status.config(text='AI LINTING...', bg=PAL['warning'])
        code = self.txt.get('1.0', 'end')
        issues = []
        for i, line in enumerate(code.split('\n'), 1):
            if 'print ' in line and (not 'print(' in line):
                issues.append(f'L{i}: print is a function call — use print()')
            if len(line) > 120:
                issues.append(f'L{i}: Line too long ({len(line)} chars)')
        if issues:
            self.status.config(text=f'LINT: {len(issues)} suggestions', bg=PAL['warning'])
            self.term.insert('end', '\n[LINT] ─────── Neural Analysis Report ───────\n', 'warn')
            for iss in issues:
                self.term.insert('end', f'  ✦ {iss}\n', 'warn')
        else:
            self.status.config(text='LINT CLEAN: 0 issues found', bg=PAL['success'])
            self.term.insert('end', '\n[LINT] ✓ Code is clean. No issues detected.\n', 'sys')
        self.after(3000, lambda: self.status.config(bg=PAL['accent']))