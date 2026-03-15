# Generated method: StartupOrchestrator._build_fstab_tab
import tkinter as tk
from tkinter import ttk, messagebox
import random

class StartupOrchestrator:
    def _build_fstab_tab(self):
        tk.Label(self.tab_fstab, text='FILESYSTEM TABLE (fstab Mount Manager)', font=('Inter', 13, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        cols = ('Device / UUID', 'Mount Point', 'Filesystem', 'Options', 'Dump', 'Pass')
        self.fstab_tree = ttk.Treeview(self.tab_fstab, columns=cols, show='headings', style='Boot.Treeview')
        for c, w in zip(cols, [200, 120, 100, 200, 50, 50]):
            self.fstab_tree.heading(c, text=c)
            self.fstab_tree.column(c, width=w, anchor='w')
        mounts = [('UUID=a1b2c3d4-...', '/', 'ext4', 'errors=remount-ro', 0, 1), ('UUID=e5f6a7b8-...', '/boot/efi', 'vfat', 'umask=0077', 0, 1), ('UUID=9c8d7e6f-...', '/home', 'btrfs', 'compress=zstd:3,noatime', 0, 2), ('tmpfs', '/tmp', 'tmpfs', 'nosuid,nodev,size=4G', 0, 0)]
        for m in mounts:
            self.fstab_tree.insert('', 'end', values=m)
        self.fstab_tree.pack(fill='both', expand=True)
        tk.Button(self.tab_fstab, text='💾 WRITE /etc/fstab', font=('Inter', 9, 'bold'), bg=PAL['danger'], fg='white', relief='flat', pady=10, command=lambda: messagebox.showinfo('fstab', 'Mount table written to /etc/fstab\nAll block devices remounted at next boot.')).pack(fill='x', pady=(15, 0))