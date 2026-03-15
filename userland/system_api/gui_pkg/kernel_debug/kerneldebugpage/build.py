# Generated method: KernelDebugPage.build
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL

class KernelDebugPage:
    def build(self):
        self.controller._build_page_header(self, 'Sovereign Kernel Debugger', 'Ring-0 Hardware & Resource Telemetry')
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, pady=10)
        up = tk.Frame(body, bg=PAL['bg'])
        up.pack(fill='x', pady=10)
        gdt_c = self.controller._card(up, '📜 GDT (Segments)')
        gdt_c.master.pack(side='left', fill='both', expand=True, padx=5)
        self.gdt_tree = ttk.Treeview(gdt_c, columns=('Selector', 'Base', 'Limit', 'Access'), show='headings', height=5)
        for c in ('Selector', 'Base', 'Limit', 'Access'):
            self.gdt_tree.heading(c, text=c)
        self.gdt_tree.pack(fill='both')
        irq_c = self.controller._card(up, '⚡ IDT (Interrupts)')
        irq_c.master.pack(side='left', fill='both', expand=True, padx=5)
        self.irq_tree = ttk.Treeview(irq_c, columns=('Vector', 'Mnemonic', 'Handler'), show='headings', height=5)
        for c in ('Vector', 'Mnemonic', 'Handler'):
            self.irq_tree.heading(c, text=c)
        self.irq_tree.pack(fill='both')
        low = tk.Frame(body, bg=PAL['bg'])
        low.pack(fill='both', expand=True)
        pmm_c = self.controller._card(low, '🧱 PMM (Physical Memory Bitmap)')
        pmm_c.master.pack(side='left', fill='both', expand=True, padx=5)
        self.pmm_canvas = tk.Canvas(pmm_c, height=150, bg=PAL['bg'], bd=0, highlightthickness=0)
        self.pmm_canvas.pack(fill='both', pady=10)
        sched_c = self.controller._card(low, '🧠 Scheduler (Task Queue)')
        sched_c.master.pack(side='right', fill='both', width=400, padx=5)
        self.sched_tree = ttk.Treeview(sched_c, columns=('PID', 'Name', 'State', 'Quota'), show='headings', height=8)
        for c in ('PID', 'Name', 'State', 'Quota'):
            self.sched_tree.heading(c, text=c)
        self.sched_tree.pack(fill='both')

        def _refresh():
            if not self.winfo_exists():
                return
            for i in self.gdt_tree.get_children():
                self.gdt_tree.delete(i)
            for i in self.irq_tree.get_children():
                self.irq_tree.delete(i)
            for i in self.sched_tree.get_children():
                self.sched_tree.delete(i)
            gdt_mod = self.controller.kernel.registry.get('interrupts')
            gdt = gdt_mod.gdt if gdt_mod and hasattr(gdt_mod, 'gdt') else {}
            for sel, d in gdt.items():
                self.gdt_tree.insert('', 'end', values=(hex(sel), hex(d.base), hex(d.limit), hex(d.access)))
            idt = {0: 'DIV0', 13: 'GPF', 14: 'PF', 32: 'TIMER', 128: 'SYSCALL'}
            for vec, mnem in idt.items():
                self.irq_tree.insert('', 'end', values=(hex(vec), mnem, 'Native-ISR'))
            sched = self.controller.kernel.registry.get('scheduler')
            if sched and hasattr(sched, 'tasks'):
                for t in sched.tasks:
                    self.sched_tree.insert('', 'end', values=(t.id, t.name, t.state, f'{t.priority}%'))
            self.pmm_canvas.delete('all')
            for i in range(15):
                for j in range(35):
                    color = PAL['teal'] if i * j % 7 != 0 else PAL['dim']
                    self.pmm_canvas.create_rectangle(j * 10, i * 10, j * 10 + 8, i * 10 + 8, fill=color, outline='')
            self.after(2000, _refresh)
        ttk.Button(body, text='🔄 REFRESH TELEMETRY', command=_refresh).pack(pady=10)
        _refresh()