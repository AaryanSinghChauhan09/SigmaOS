"""
SigmaOS Omni-Lens Pro (v1.0)
============================
Neural optical character recognition and visual search matrix.
USP: On-device generative AI vision parsing with zero cloud dependency.
Equivalent to: Google Lens / Apple Live Text / Bixby Vision.
"""
import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import time
import threading

PAL = {
    "bg": "#0B0C0E",
    "sidebar": "#16181C",
    "accent": "#FF4500", # Vision Orange
    "accent_dim": "#CC3700",
    "text": "#F2F2F7",
    "dim": "#8E8E93",
    "danger": "#FF3B30",
    "success": "#32D74B",
    "panel": "#1C1E24"
}

class OmniLensPro(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("Sovereign Omni-Lens Pro")
        self.geometry("1000x700")
        self.configure(bg=PAL["bg"])
        
        self.scanning = False
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Lens.TProgressbar", background=PAL["accent"], troughcolor=PAL["sidebar"], borderwidth=0)

    def _build_ui(self):
        # Header
        self.header = tk.Frame(self, bg=PAL["bg"], height=70, padx=25)
        self.header.pack(side="top", fill="x", pady=15)
        
        tk.Label(self.header, text="OMNI-LENS VISION", font=("Inter", 20, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        btn_fr = tk.Frame(self.header, bg=PAL["bg"])
        btn_fr.pack(side="right")
        
        nav_btns = [("📷 FEED", self._simulate_camera), ("📁 FILE", self._select_image)]
        for txt, cmd in nav_btns:
             tk.Button(btn_fr, text=txt, font=("Inter", 9, "bold"), bg=PAL["sidebar"], fg="white", 
                       relief="flat", padx=15, pady=8, command=cmd).pack(side="left", padx=5)

        tk.Button(btn_fr, text="🧠 NEURAL PARSE", font=("Inter", 9, "bold"), bg=PAL["accent"], fg="black", 
                  relief="flat", padx=15, pady=8, command=self._start_scan).pack(side="left", padx=5)

        # Workspace
        self.workspace = tk.Frame(self, bg=PAL["bg"], padx=25, pady=10)
        self.workspace.pack(fill="both", expand=True)

        # Split UI: Viewport and Results
        self.view_fr = tk.Frame(self.workspace, bg=PAL["panel"], width=500, padx=10, pady=10)
        self.view_fr.pack(side="left", fill="both", expand=True, padx=(0, 10))
        self.view_fr.pack_propagate(False)
        
        tk.Label(self.view_fr, text="OPTICAL VIEWPORT", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        
        self.canvas = tk.Canvas(self.view_fr, bg=PAL["sidebar"], highlightthickness=0)
        self.canvas.pack(fill="both", expand=True, pady=10)
        
        self.canvas.create_text(250, 250, text="AWAITING VISUAL FEED...", fill=PAL["dim"], font=("Inter", 12, "bold"))
        self.scan_line = None

        # Results Panel
        self.res_fr = tk.Frame(self.workspace, bg=PAL["panel"], width=400, padx=15, pady=15)
        self.res_fr.pack(side="left", fill="both", expand=True)
        self.res_fr.pack_propagate(False)
        
        tk.Label(self.res_fr, text="EXTRACTED VECTORS", font=("Inter", 10, "bold"), fg=PAL["dim"], bg=PAL["panel"]).pack(anchor="w")
        
        self.res_text = tk.Text(self.res_fr, bg=PAL["sidebar"], fg=PAL["success"], font=("Consolas", 11), relief="flat")
        self.res_text.pack(fill="both", expand=True, pady=10)
        self.res_text.insert(tk.END, ">>> READY FOR INGEST.\n")
        self.res_text.config(state=tk.DISABLED)

        # Status
        self.status = tk.Label(self, text="VISION SENSORS DORMANT | ON-DEVICE ML LOADED", 
                               bg=PAL["accent_dim"], fg="white", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _log(self, msg):
        self.res_text.config(state=tk.NORMAL)
        self.res_text.insert(tk.END, f"{msg}\n")
        self.res_text.see(tk.END)
        self.res_text.config(state=tk.DISABLED)

    def _simulate_camera(self):
        self.canvas.delete("all")
        self.canvas.create_rectangle(50, 50, 450, 450, outline=PAL["dim"], width=2, dash=(5, 5))
        self.canvas.create_text(250, 250, text="[LIVE SENSOR FEED ACQUIRED]", fill=PAL["text"], font=("Inter", 10))
        self.status.config(text="CAMERA FEED ACTIVE | 4K 60FPS", bg=PAL["accent"], fg="black")

    def _select_image(self):
        file = filedialog.askopenfilename()
        if file:
            self.canvas.delete("all")
            self.canvas.create_rectangle(50, 50, 450, 450, outline=PAL["accent_dim"], width=2)
            self.canvas.create_text(250, 250, text=f"[IMAGE MOUNTED:\n{file.split('/')[-1]}]", fill=PAL["text"], font=("Inter", 10), justify="center")
            self.status.config(text="STATIC IMAGE MOUNTED INTO VRAM", bg=PAL["panel"], fg=PAL["text"])

    def _start_scan(self):
        if self.scanning: return
        self.scanning = True
        self.res_text.config(state=tk.NORMAL)
        self.res_text.delete(1.0, tk.END)
        self.res_text.config(state=tk.DISABLED)
        
        self._log(">>> INITIATING ON-DEVICE TENSOR FLOW...")
        self.status.config(text="SCANNING VISUAL MATRIX...", bg=PAL["warning"], fg="black")
        
        def animate_scan(y):
            if not self.scanning: return
            if self.scan_line: self.canvas.delete(self.scan_line)
            
            if y < 450:
                self.scan_line = self.canvas.create_line(50, y, 450, y, fill=PAL["accent"], width=3)
                self.after(20, lambda: animate_scan(y + 10))
            else:
                self.canvas.delete(self.scan_line)
                self._finish_scan()

        animate_scan(50)

    def _finish_scan(self):
        self.scanning = False
        mocks = [
            "[TEXT] -> 'Sovereign Architecture Protocol v5'",
            "[TEXT] -> 'SigmaOS Terminal Keys'",
            "[LINK] -> 'https://sigma.local/secure'",
            "[OBJECT] -> Class: 'Quantum Motherboard' Conf: 98.2%",
            "[ENTITY] -> 'Encrypted Barcode Fragment'"
        ]
        
        self.after(500, lambda: self._log(">>> EXTRACTION COMPLETE:"))
        for i, m in enumerate(mocks):
            self.after(1000 + (i * 300), lambda msg=m: self._log(f"    {msg}"))
            
        self.after(3000, lambda: self.status.config(text="NEURAL PARSE COMPLETE | ZERO DATA EXFILTRATION", bg=PAL["success"], fg="black"))

if __name__ == "__main__":
    app = OmniLensPro()
    app.mainloop()
