# Generated method: ForensicVault._scan_worker
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import os, hashlib, time, threading

class ForensicVault:
    def _scan_worker(self):
        root = self.monitored_paths[0]
        files_count = 0
        for r, d, fnames in os.walk(root):
            if '__pycache__' in r or '.git' in r:
                continue
            for f in fnames:
                fpath = os.path.join(r, f)
                try:
                    h = self._get_hash(fpath)
                    rel = os.path.relpath(fpath, root)
                    if rel in self.file_hashes and self.file_hashes[rel] != h:
                        self._log(f'TAMPER DETECTED: {rel}', 'CRITICAL')
                        self.tree.insert('', '0', text=rel, values=('MODIFIED',))
                    else:
                        self.file_hashes[rel] = h
                    files_count += 1
                except:
                    pass
        self._log(f'SCAN COMPLETED. {files_count} NODES VERIFIED.', 'SUCCESS')
        self._running = False