import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD

class AutomationHubPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "Cosmos AI-OS Automation", "Programmable Interfaces & Neuro-Top (v1.0)")
        
        body = tk.Frame(self, bg=PAL["bg"], padx=20, pady=10)
        body.pack(fill="both", expand=True)
        
        # Connect to Kernel Logic
        def _sync_9p():
            v9 = self.controller.kernel.registry.get("virtio_9p")
            if v9:
                res = v9.mount_host_folder()
                self.controller._log_voice(res["message"])

        def _run_gc():
            lisp = self.controller.kernel.registry.get("lisp")
            if lisp:
                res = lisp.collect_garbage()
                self.controller._log_voice(res)
        
        # Top Row
        top = tk.Frame(body, bg=PAL["bg"])
        top.pack(fill="x", pady=5)
        
        # Serial Card
        ser_fr = self.controller._card(top, "Serial Console (COM1)")
        ser_fr.master.pack(side="left", fill="both", expand=True, padx=5)
        self.ser_log = tk.Text(ser_fr, bg="black", fg=PAL["green"], font=("Consolas", 9), height=10, bd=0)
        self.ser_log.pack(fill="both", expand=True)
        
        # Neuro-Top Card
        neuro_fr = self.controller._card(top, "Neuro-Top (AI Heatmap)")
        neuro_fr.master.pack(side="left", fill="both", expand=True, padx=5)
        self.n_canv = tk.Canvas(neuro_fr, bg="black", height=150, bd=0, highlightthickness=0)
        self.n_canv.pack(fill="both", expand=True)
        
        def _draw_heatmap():
            if not self.winfo_exists(): return
            self.n_canv.delete("all")
            import random
            for y in range(0, 150, 15):
                for x in range(0, 200, 15):
                    alpha = random.randint(50, 255)
                    color = f"#{0:02x}{alpha:02x}{alpha:02x}" # Teal-ish activation
                    self.n_canv.create_rectangle(x, y, x+14, y+14, fill=color, outline="")
            self.after(1500, _draw_heatmap)
        _draw_heatmap()

        # 9P Status Card
        v9_fr = self.controller._card(top, "Virtio-9P (Host Sync)")
        v9_fr.master.pack(side="left", fill="both", expand=True, padx=5)
        tk.Label(v9_fr, text="Status: SYNCED", fg=PAL["green"], bg=PAL["bg2"], font=FONT_BOLD).pack(pady=10)
        ttk.Button(v9_fr, text="RE-SYNC HOST", command=_sync_9p).pack(pady=5)

        # Bottom Row
        bot = tk.Frame(body, bg=PAL["bg"])
        bot.pack(fill="both", expand=True, pady=10)
        
        # Lisp REPL
        lisp_fr = self.controller._card(bot, "Sovereign Lisp REPL (Live-Patching)")
        lisp_fr.master.pack(side="left", fill="both", expand=True, padx=5)
        self.lisp_out = tk.Text(lisp_fr, bg=PAL["bg3"], fg=PAL["teal"], font=("JetBrains Mono", 10), height=10, bd=0)
        self.lisp_out.pack(fill="both", expand=True)
        self.lisp_out.insert("1.0", ";; Sovereign Lisp v1.0\n> ")
        
        self.lisp_ent = tk.Entry(lisp_fr, bg=PAL["bg"], fg="white", bd=0, insertbackground="white", font=("JetBrains Mono", 10))
        self.lisp_ent.pack(fill="x", pady=5)
        
        def _eval_lisp(e=None):
            cmd = self.lisp_ent.get()
            lisp = self.controller.kernel.registry.get("lisp")
            if lisp:
                res = lisp.eval(cmd)
                self.lisp_out.insert("end", f"\n{cmd}\n=> {res}\n> ")
                self.lisp_ent.delete(0, "end")
                self.lisp_out.see("end")

        self.lisp_ent.bind("<Return>", _eval_lisp)
        ttk.Button(lisp_fr, text="TRIGGER GC", command=_run_gc).pack(side="right")
        
        # Refresh Logic
        def _refresh_automation():
            if not self.winfo_exists(): return
            ser = self.controller.kernel.registry.get("serial")
            if ser:
                self.ser_log.delete("1.0", "end")
                self.ser_log.insert("1.0", ser.get_serial_logs())
                self.ser_log.see("end")
            self.after(2000, _refresh_automation)

        _refresh_automation()
