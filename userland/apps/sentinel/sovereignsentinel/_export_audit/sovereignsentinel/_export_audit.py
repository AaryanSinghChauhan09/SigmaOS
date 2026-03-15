# Generated method: SovereignSentinel._export_audit
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess

class SovereignSentinel:
    def _export_audit(self):
        from tkinter import filedialog
        f = filedialog.asksaveasfilename(defaultextension='.txt', title='Export Audit Log')
        if f:
            with open(f, 'w') as fp:
                fp.write('SigmaOS Sovereign Security Guardian - Audit Log Export\n')
                fp.write('=' * 60 + '\n')
                for row in self.audit_tree.get_children():
                    fp.write('  |  '.join((str(v) for v in self.audit_tree.item(row, 'values'))) + '\n')
            messagebox.showinfo('Export', f'Audit log exported to:\n{f}')