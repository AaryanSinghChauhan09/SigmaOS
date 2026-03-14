"""
SigmaOS NCERT Human Physiology Hub v1.0
=======================================
Advanced Biological Telemetry for Classes 11-12
100% Native Python | Standard Library | Sovereign Aesthetics
"""
import tkinter as tk
from tkinter import ttk, messagebox
import math, random

PAL = {
    "bg": "#06070B",
    "panel": "#0E121C",
    "heart": "#FF3B30",
    "neural": "#58A6FF",
    "text": "#E2E8F0",
    "accent": "#EC4899"
}

class PhysiologyHub(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("SigmaOS • Human Physiology Hub")
        self.geometry("1000x800")
        self.configure(bg=PAL["bg"])
        
        self._build_ui()

    def _build_ui(self):
        # Header
        hdr = tk.Frame(self, bg=PAL["panel"], height=80)
        hdr.pack(fill="x")
        tk.Label(hdr, text="💓 HUMAN PHYSIOLOGY MASTER", font=("Segoe UI Bold", 20), fg=PAL["heart"], bg=PAL["panel"]).pack(pady=20)

        # Tabs for different systems
        tabs = ttk.Notebook(self)
        tabs.pack(fill="both", expand=True, padx=20, pady=20)

        # Tab 1: Circulatory
        circ = tk.Frame(tabs, bg=PAL["bg"])
        tabs.add(circ, text="CIRCULATORY")
        self._setup_circ(circ)

        # Tab 2: Respiratory
        resp = tk.Frame(tabs, bg=PAL["bg"])
        tabs.add(resp, text="RESPIRATORY")
        self._setup_resp(resp)

        # Tab 3: Neural
        neur = tk.Frame(tabs, bg=PAL["bg"])
        tabs.add(neur, text="NEURAL")
        self._setup_neur(neur)

    def _setup_circ(self, master):
        tk.Label(master, text="CARDIAC CYCLE SIMULATOR", font=("Segoe UI Bold", 12), fg=PAL["heart"], bg=PAL["bg"]).pack(pady=10)
        self.ecg_canvas = tk.Canvas(master, bg="#000", height=200, highlightthickness=1, highlightbackground=PAL["heart"])
        self.ecg_canvas.pack(fill="x", padx=40, pady=20)
        
        tk.Button(master, text="GENERATE ECG SCAN", command=self._draw_ecg, bg=PAL["heart"], fg="white", relief="flat", padx=20).pack()
        
        info = tk.Label(master, text="P-Wave: Atrial Depolarization\nQRS: Ventricular Depolarization\nT-Wave: Ventricular Repolarization", 
                        fg=PAL["text"], bg=PAL["bg"], justify="left", pady=20)
        info.pack()

    def _draw_ecg(self):
        self.ecg_canvas.delete("all")
        points = [(0, 100)]
        x = 0
        for _ in range(10):
            # P wave
            for i in range(20): points.append((x+i, 100 - 10*math.sin(i*0.15))); 
            x += 20
            # QRS
            points.append((x, 100)); points.append((x+5, 110)); points.append((x+10, 40)); points.append((x+15, 110)); points.append((x+20, 100))
            x += 20
            # T wave
            for i in range(30): points.append((x+i, 100 - 15*math.sin(i*0.1)));
            x += 50
        
        self.ecg_canvas.create_line(points, fill="#00FF00", width=2)

    def _setup_resp(self, master):
        tk.Label(master, text="LUNG CAPACITY FLOWMETER", font=("Segoe UI Bold", 12), fg=PAL["neural"], bg=PAL["bg"]).pack(pady=10)
        
        fr = tk.Frame(master, bg=PAL["bg"])
        fr.pack(pady=20)
        
        tk.Label(fr, text="TV (ml):", fg="white", bg=PAL["bg"]).grid(row=0, column=0)
        self.tv = tk.Entry(fr, width=10); self.tv.insert(0, "500"); self.tv.grid(row=0, column=1, padx=5)
        
        tk.Label(fr, text="IRV (ml):", fg="white", bg=PAL["bg"]).grid(row=1, column=0)
        self.irv = tk.Entry(fr, width=10); self.irv.insert(0, "2500"); self.irv.grid(row=1, column=1, padx=5)
        
        tk.Button(master, text="CALCULATE VITAL CAPACITY", command=self._calc_vc, bg=PAL["neural"], fg="white", relief="flat").pack(pady=10)
        self.vc_res = tk.Label(master, text="VC: -- ml", fg=PAL["text"], bg=PAL["bg"], font=("Segoe UI Bold", 14))
        self.vc_res.pack()

    def _calc_vc(self):
        try:
            vc = int(self.tv.get()) + int(self.irv.get()) + 1000 # ERV=1000
            self.vc_res.config(text=f"VC: {vc} ml | Inspiratory Capacity: {int(self.tv.get())+int(self.irv.get())} ml")
        except: pass

    def _setup_neur(self, master):
        tk.Label(master, text="REFLEX ARC & SYNAPSE", font=("Segoe UI Bold", 12), fg=PAL["accent"], bg=PAL["bg"]).pack(pady=10)
        c = tk.Canvas(master, bg="#050510", height=300, highlightthickness=0)
        c.pack(fill="both", expand=True, padx=40)
        
        # Draw basic neuron
        c.create_oval(100, 100, 160, 160, fill="#1A1E30", outline=PAL["neural"], width=2)
        c.create_line(160, 130, 400, 130, fill=PAL["neural"], width=4)
        c.create_text(130, 200, text="SOMA", fill="white")
        c.create_text(280, 150, text="AXON", fill="white")
        
        # Nerve Impulse animation logic would go here
        tk.Button(master, text="SIMULATE IMPULSE", command=lambda: messagebox.showinfo("Neural Pro", "Propagation: Saltatory Conduction Active"), 
                  bg=PAL["accent"], fg="white", relief="flat").pack(pady=10)

if __name__ == "__main__":
    PhysiologyHub().mainloop()
