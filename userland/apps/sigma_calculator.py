"""
SigmaOS Scientific Calculator v1.0
100% stdlib — tkinter only, zero 3rd-party deps.
"""
import tkinter as tk
from tkinter import ttk
import math, cmath, re, json, os
from typing import Any, List

try:
    from sigma_core.ui.fluid_design import ICONS # type: ignore
except ImportError:
    ICONS = {}

PAL = {"bg":"#0D0F18","panel":"#13162A","card":"#1A1E30","accent":"#6C63FF",
       "success":"#00D26A","danger":"#FF4D4D","text":"#E8E8F0","dim":"#9090A0",
       "border":"#2A2D45","btn":"#1F2338","btnH":"#2E3252"}

HISTORY_FILE = os.path.join(os.path.dirname(__file__), "..", "..", "tmp", "calc_history.json")

class SigmaCalculator(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("SigmaOS Scientific Calculator")
        self.geometry("540x720"); self.resizable(False, False)
        self.configure(bg=PAL["bg"])
        self._expr = ""; self._memory = 0.0; self._history: List[str] = []
        self._mode = tk.StringVar(value="DEC")
        
        # UI Proxies
        self._hist_lbl: Any = None
        self._disp: Any = None
        self._mem_lbl: Any = None
        self._hist_box: Any = None

        self._load_history()
        self._build()

    def _load_history(self):
        try:
            os.makedirs(os.path.dirname(HISTORY_FILE), exist_ok=True)
            if os.path.exists(HISTORY_FILE):
                with open(HISTORY_FILE) as f:
                    self._history = json.load(f)
        except Exception: pass

    def _save_history(self):
        try:
            with open(HISTORY_FILE, "w") as f:
                json.dump(self._history[-50:], f)
        except Exception: pass

    def _build(self):
        # Title bar
        hdr = tk.Frame(self, bg=PAL["panel"], height=50)
        hdr.pack(fill="x"); hdr.pack_propagate(False)
        tk.Label(hdr, text=f"{ICONS.get('calculator', '🧮')} SIGMA CALCULATOR", fg=PAL["accent"],
                 bg=PAL["panel"], font=("Segoe UI Bold", 13)).pack(side="left", padx=18, pady=10)
        for m in ("DEC","HEX","BIN","OCT"):
            rb = tk.Radiobutton(hdr, text=m, variable=self._mode, value=m,
                                fg=PAL["dim"], bg=PAL["panel"], selectcolor=PAL["card"],
                                activebackground=PAL["panel"], font=("Segoe UI",8),
                                command=self._mode_changed)
            rb.pack(side="right", padx=4)

        # Display
        disp_fr = tk.Frame(self, bg=PAL["card"], highlightthickness=1,
                           highlightbackground=PAL["border"])
        disp_fr.pack(fill="x", padx=12, pady=8)
        self._hist_lbl = tk.Label(disp_fr, text="", fg=PAL["dim"], bg=PAL["card"],
                                  font=("Cascadia Code",9), anchor="e")
        self._hist_lbl.pack(fill="x", padx=12, pady=(8,0))
        self._disp = tk.Label(disp_fr, text="0", fg=PAL["text"], bg=PAL["card"],
                              font=("Cascadia Code",26), anchor="e")
        self._disp.pack(fill="x", padx=12, pady=(0,8))
        self._mem_lbl = tk.Label(disp_fr, text="M: 0", fg=PAL["dim"], bg=PAL["card"],
                                 font=("Segoe UI",8), anchor="w")
        self._mem_lbl.pack(fill="x", padx=12, pady=(0,6))

        # Buttons grid
        grid = tk.Frame(self, bg=PAL["bg"])
        grid.pack(fill="both", expand=True, padx=12, pady=(0,12))

        ROWS = [
            [("sin","fn"),("cos","fn"),("tan","fn"),("log","fn"),("ln","fn")],
            [("x²","fn"),("√","fn"),("π","fn"),("e","fn"),("C","clr")],
            [("MC","mem"),("MR","mem"),("M+","mem"),("M-","mem"),("MS","mem")],
            [("7","num"),("8","num"),("9","num"),("÷","op"),("(",  "op")],
            [("4","num"),("5","num"),("6","num"),("×","op"),(")",  "op")],
            [("1","num"),("2","num"),("3","num"),("−","op"),("%","op")],
            [("0","num"),(".","num"),("±","fn"), ("+","op"),("=","eq")],
        ]
        COLOR_MAP={"num":PAL["btn"],"op":"#252848","fn":"#1C2040",
                   "clr":PAL["danger"],"eq":PAL["accent"],"mem":"#1A2535"}
        for r,row in enumerate(ROWS):
            for c,( label, typ) in enumerate(row):
                bg=COLOR_MAP.get(typ,PAL["btn"])
                btn=tk.Button(grid,text=label,bg=bg,fg=PAL["text"],
                               font=("Segoe UI",12),relief="flat",
                               command=lambda l=label:self._press(l))
                btn.grid(row=r,column=c,padx=3,pady=3,sticky="nsew",ipady=10)
                btn.bind("<Enter>",lambda e,b=btn:b.config(bg=PAL["btnH"]))
                btn.bind("<Leave>",lambda e,b=btn,c=bg:b.config(bg=c))
        for i in range(5): grid.columnconfigure(i,weight=1)
        for i in range(7): grid.rowconfigure(i,weight=1)

        # History panel
        hpanel=tk.Frame(self,bg=PAL["panel"],height=90)
        hpanel.pack(fill="x",padx=12,pady=(0,8)); hpanel.pack_propagate(False)
        tk.Label(hpanel,text="HISTORY",fg=PAL["dim"],bg=PAL["panel"],
                 font=("Segoe UI",8,"bold")).pack(anchor="w",padx=10,pady=(6,2))
        self._hist_box=tk.Text(hpanel,bg=PAL["panel"],fg=PAL["dim"],
                                font=("Cascadia Code",8),borderwidth=0,height=3)
        self._hist_box.pack(fill="x",padx=10); self._refresh_history()

    def _press(self,label):
        OP_MAP={"×":"*","÷":"/","−":"-"}
        if label=="C": self._expr=""; self._update("0"); return
        if label=="=": self._evaluate(); return
        if label=="±":
            if self._expr and not self._expr.startswith("-"): self._expr="-"+self._expr
            elif self._expr.startswith("-"): self._expr=self._expr[1:]
            self._update(self._expr or "0"); return
        if label=="MC": self._memory=0.0; self._mem_lbl.config(text="M: 0"); return
        if label=="MR": self._expr+=str(self._memory); self._update(self._expr); return
        if label=="MS": self._memory=self._safe_eval(); self._mem_lbl.config(text=f"M: {self._memory}"); return
        if label=="M+": self._memory+=self._safe_eval(); self._mem_lbl.config(text=f"M: {self._memory}"); return
        if label=="M-": self._memory-=self._safe_eval(); self._mem_lbl.config(text=f"M: {self._memory}"); return
        # Functions
        fn_map={"sin":"math.sin(math.radians(","cos":"math.cos(math.radians(",
                "tan":"math.tan(math.radians(","log":"math.log10(",
                "ln":"math.log(","√":"math.sqrt(","x²":"(","π":str(math.pi),"e":str(math.e)}
        if label in fn_map:
            if label=="x²": self._expr=f"({self._expr})**2" if self._expr else "0**2"
            elif label in("π","e"): self._expr+=fn_map[label]
            else: self._expr+=fn_map[label]; self._update(self._expr); return
        else:
            self._expr+=OP_MAP.get(label,label)
        self._update(self._expr)

    def _safe_eval(self):
        try: return float(eval(self._expr,{"__builtins__":{},"math":math},{}))
        except Exception: return 0.0

    def _evaluate(self):
        try:
            raw=self._expr.replace("^","**")
            result=eval(raw,{"__builtins__":{},"math":math,"cmath":cmath},{})
            result=round(float(result.real if isinstance(result,complex) else result),10)
            # Mode conversion
            mode=self._mode.get()
            if mode=="HEX": display=hex(int(result)).upper()
            elif mode=="BIN": display=bin(int(result))
            elif mode=="OCT": display=oct(int(result))
            else: display=str(result).rstrip("0").rstrip(".")  if "." in str(result) else str(result)
            entry=f"{self._expr} = {display}"
            self._history.append(entry)
            self._save_history(); self._refresh_history()
            self._hist_lbl.config(text=self._expr)
            self._expr=str(result); self._update(display)
        except Exception as ex:
            self._update(f"ERR: {ex}"); self._expr=""

    def _update(self,txt):
        self._disp.config(text=txt[-22:] if len(txt)>22 else txt)

    def _mode_changed(self):
        try:
            val=float(self._expr) if self._expr else 0
            m=self._mode.get()
            if m=="HEX": self._update(hex(int(val)).upper())
            elif m=="BIN": self._update(bin(int(val)))
            elif m=="OCT": self._update(oct(int(val)))
            else: self._update(str(val))
        except Exception: pass

    def _refresh_history(self):
        self._hist_box.delete("1.0","end")
        for h in self._history[-5:]: self._hist_box.insert("end",h+"\n")

def launch(kernel=None):
    SigmaCalculator(kernel).mainloop()

if __name__=="__main__":
    launch()
