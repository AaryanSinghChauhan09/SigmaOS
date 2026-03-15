# Generated method: AetherNetMapper._start_scan
import tkinter as tk
from tkinter import ttk, messagebox
import socket
import threading
import random

class AetherNetMapper:
    def _start_scan(self):
        target = self.ip_entry.get().strip()
        if not target:
            messagebox.showerror('Error', 'Enter a valid Target Vector.')
            return
        if self.scanning:
            return
        self.scanning = True
        self.terminal.delete(1.0, tk.END)
        self._log(f'INITIATING PORT RESONANCE ON: {target}')

        def scan_worker():
            ports = [21, 22, 23, 25, 53, 80, 110, 135, 139, 143, 443, 445, 993, 3306, 3389, 8080]
            open_count = 0
            for port in ports:
                if not self.scanning:
                    break
                self._log(f'TESTING NEURAL TUNNEL -> PORT {port}...')
                sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                sock.settimeout(0.2)
                try:
                    res = sock.connect_ex((target, port))
                    if res == 0:
                        self._log(f'[!] ANOMALY: PORT {port} OPEN/RESONATING', PAL['danger'])
                        open_count += 1
                    else:
                        self._log(f'PORT {port} SECURE (CLOSED/FILTERED)', PAL['dim'])
                except:
                    self._log(f'PORT {port} REFUSED CONNECTION', PAL['dim'])
                finally:
                    sock.close()
                time.sleep(0.3)
            self._log(f'RESONANCE SCAN COMPLETE. {open_count} VECTORS EXPOSED.')
            self.scanning = False
        threading.Thread(target=scan_worker, daemon=True).start()