# Generated method: SovereignDeviceManager._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class SovereignDeviceManager:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='OMNI-DEVICE KERNEL MANAGER', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        nav_btns = [('⚡ RE-ROUTE DMA', self._route_dma), ('🛡️ SANDBOX DRIVERS', self._sandbox_drivers)]
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.prin_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=280, padx=15, pady=15)
        self.prin_fr.pack(side='left', fill='y', padx=(0, 20))
        self.prin_fr.pack_propagate(False)
        tk.Label(self.prin_fr, text='OS HARDWARE PRINCIPLES', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        principles = [('Interrupts (IRQ)', 'Hardware lines that signal the CPU to halt execution and handle events instantaneously.'), ('Direct Memory Access (DMA)', 'Allows hardware subsystems to access main system memory independently of the CPU.'), ('Hardware Abstraction (HAL)', 'Software layer isolating kernel logic from specific hardware architectures.')]
        for name, desc in principles:
            f = tk.Frame(self.prin_fr, bg=PAL['sidebar'], pady=10, padx=10)
            f.pack(fill='x', pady=5)
            tk.Label(f, text=f'💠 {name}', font=('Inter', 9, 'bold'), fg=PAL['text'], bg=PAL['sidebar']).pack(anchor='w')
            tk.Label(f, text=desc, font=('Inter', 8), fg=PAL['dim'], bg=PAL['sidebar'], wraplength=220, justify='left').pack(anchor='w', pady=(5, 0))
        self.tree_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.tree_fr.pack(side='left', fill='both', expand=True)
        tk.Label(self.tree_fr, text='HARDWARE ABSTRACTION LAYER (HAL) TOPOLOGY', font=('Inter', 12, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        cols = ('Hardware Component', 'HAL Address', 'Vector (IRQ/DMA)', 'State')
        self.tree = ttk.Treeview(self.tree_fr, columns=cols, show='headings', style='Device.Treeview')
        widths = [250, 120, 150, 200]
        for c, w in zip(cols, widths):
            self.tree.heading(c, text=c.upper())
            self.tree.column(c, width=w, anchor='w')
        for dev in self.devices:
            self.tree.insert('', 'end', values=dev)
        self.tree.pack(fill='both', expand=True)
        self.tree.bind('<Double-1>', self._inspect_device)
        self.status = tk.Label(self, text='HAL KERNEL INTERFACE IDLE | 0 HARDWARE FAULTS DETECTED', bg=PAL['accent_dim'], fg='black', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')