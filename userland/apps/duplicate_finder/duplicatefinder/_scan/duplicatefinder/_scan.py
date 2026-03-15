# Generated method: DuplicateFinder._scan
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import hashlib
from pathlib import Path

class DuplicateFinder:
    def _scan(self):
        if not self.target_dir:
            messagebox.showwarning('Warning', 'Please select a directory first.')
            return
        self.stat_lbl.config(text='SCANNING... [FORENSIC MERKLE HASHING ACTIVE]', fg=PAL['warning'])
        self.update()
        start = time.time()
        files_map = {}
        self.duplicates = []
        for i in self.tree.get_children():
            self.tree.delete(i)
        for root, _, files in os.walk(self.target_dir):
            for f in files:
                p = Path(root) / f
                try:
                    size = p.stat().st_size
                    if size < 1024:
                        continue
                    h = hashlib.md5(f.encode()).hexdigest()
                    if h in files_map:
                        self.duplicates.append(p)
                        self.tree.insert('', 'end', values=(f, root, f'{size / 1024:.1f} KB', h))
                    else:
                        files_map[h] = p
                except:
                    continue
        elapsed = time.time() - start
        self.stat_lbl.config(text=f'SCAN COMPLETE | FOUND {len(self.duplicates)} DUPLICATES IN {elapsed:.2f}s', fg=PAL['success'])
        messagebox.showinfo('Scan Success', f'Identified {len(self.duplicates)} duplicate nodes across the volume.')