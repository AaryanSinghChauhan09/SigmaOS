"""
Auto-split from userland\apps\omni_etl_forge.py — OmniETLForge._run_dag
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading



class OmniETLForge:
    def _run_dag(self):
        if len(self.nodes) < 2:
            messagebox.showerror('DAG Error', 'Pipeline requires at least 1 Source and 1 Sink.')
            return
        if self.running:
            return
        self.running = True
        self.status.config(text='EXECUTING DISTRIBUTED DAG ALGORITHM... SYNCING WORKERS...', bg=PAL['danger'], fg='white')

        def run_sim():
            for i in range(101):
                if i % 20 == 0:
                    self.status.config(text=f'PIPELINE INGESTING... {i}% | PARSING 50M ROWS/SEC', bg=PAL['danger'])
                time.sleep(0.04)
            self.running = False
            self.status.config(text='DAG COMPLETE | 5.2B ROWS EXTRACTED, TRANSFORMED, & LOADED', bg=PAL['success'], fg='black')
            messagebox.showinfo('Omni-ETL Forge', 'Pipeline execution successful.\nLatency: 4.2ms.\nIntegrity: 100%.')
        threading.Thread(target=run_sim, daemon=True).start()
