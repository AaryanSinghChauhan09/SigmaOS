"""
SigmaOS Logic Gate Simulator v1.0
Visual digital logic: AND/OR/NOT/NAND/NOR/XOR/XNOR
Inspired by Logic Circuit Simulator Pro — 100% stdlib/tkinter
"""
import tkinter as tk
from tkinter import ttk

PAL={"bg":"#0B0D17","panel":"#12142A","card":"#1A1C2E","accent":"#6C63FF",
     "on":"#00D26A","off":"#FF4D4D","wire":"#4A4E82","text":"#E8E8F0","dim":"#9090A0"}

GATES = {
    "AND":  lambda a,b: a and b,
    "OR":   lambda a,b: a or b,
    "NOT":  lambda a,b: not a,
    "NAND": lambda a,b: not (a and b),
    "NOR":  lambda a,b: not (a or b),
    "XOR":  lambda a,b: (a or b) and not (a and b),
    "XNOR": lambda a,b: not ((a or b) and not (a and b)),
    "BUF":  lambda a,b: a,
}

def full_adder(A,B,Cin):
    S = A^B^Cin; Cout = (A&B)|(B&Cin)|(A&Cin)
    return S,Cout

def half_adder(A,B):
    return A^B, A&B

class LogicSimulator(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.title("SigmaOS Logic Gate Simulator")
        self.geometry("900x620"); self.configure(bg=PAL["bg"])
        self._A=tk.IntVar(value=0); self._B=tk.IntVar(value=0)
        self._Cin=tk.IntVar(value=0)
        self._build()

    def _build(self):
        hdr=tk.Frame(self,bg=PAL["panel"],height=50); hdr.pack(fill="x"); hdr.pack_propagate(False)
        tk.Label(hdr,text="⚡ LOGIC GATE SIMULATOR",fg=PAL["accent"],bg=PAL["panel"],
                 font=("Segoe UI Bold",14)).pack(side="left",padx=18,pady=10)

        nb=ttk.Notebook(self); nb.pack(fill="both",expand=True,padx=10,pady=10)
        self._build_single(nb); self._build_truth(nb); self._build_adder(nb)
        self._build_combinational(nb)

    def _build_single(self,nb):
        tab=tk.Frame(nb,bg=PAL["bg"]); nb.add(tab,text="  🔌 Single Gate  ")
        # Inputs
        inp=tk.Frame(tab,bg=PAL["bg"]); inp.pack(pady=20)
        for lbl,var in (("Input A",self._A),("Input B",self._B)):
            fr=tk.Frame(inp,bg=PAL["bg"]); fr.pack(side="left",padx=20)
            tk.Label(fr,text=lbl,fg=PAL["dim"],bg=PAL["bg"],font=("Segoe UI",10)).pack()
            tk.Button(fr,textvariable=tk.StringVar(value="1" if var.get() else "0"),
                      bg=PAL["on"] if var.get() else PAL["off"],fg="white",
                      font=("Segoe UI Bold",14),relief="flat",width=4,
                      command=lambda v=var: self._toggle(v)).pack(pady=4)

        # Gate buttons
        gf=tk.Frame(tab,bg=PAL["bg"]); gf.pack(pady=10)
        tk.Label(gf,text="Select Gate:",fg=PAL["dim"],bg=PAL["bg"],
                 font=("Segoe UI",10)).pack(side="left",padx=8)
        self._gate_var=tk.StringVar(value="AND")
        for g in GATES:
            rb=tk.Radiobutton(gf,text=g,variable=self._gate_var,value=g,
                              fg=PAL["text"],bg=PAL["bg"],selectcolor=PAL["card"],
                              activebackground=PAL["bg"],font=("Segoe UI",9),
                              command=self._evaluate)
            rb.pack(side="left",padx=4)

        # Output visual
        out_fr=tk.Frame(tab,bg=PAL["card"],padx=30,pady=20,
                        highlightthickness=1,highlightbackground=PAL["accent"])
        out_fr.pack(pady=20,padx=40,fill="x")
        tk.Label(out_fr,text="OUTPUT",fg=PAL["dim"],bg=PAL["card"],
                 font=("Segoe UI",9,"bold")).pack()
        self._out_lbl=tk.Label(out_fr,text="0",fg=PAL["off"],bg=PAL["card"],
                                font=("Cascadia Code",52))
        self._out_lbl.pack()
        self._out_txt=tk.Label(out_fr,text="LOW",fg=PAL["dim"],bg=PAL["card"],
                                font=("Segoe UI",12))
        self._out_txt.pack()

        # Input toggles (proper)
        ctrl=tk.Frame(tab,bg=PAL["bg"]); ctrl.pack(pady=8)
        self._a_btn=tk.Button(ctrl,text=f"A = {self._A.get()}",
                               bg=PAL["on"] if self._A.get() else PAL["off"],fg="white",
                               font=("Segoe UI Bold",10),relief="flat",padx=16,pady=6,
                               command=lambda: self._toggle_btn("A"))
        self._a_btn.pack(side="left",padx=8)
        self._b_btn=tk.Button(ctrl,text=f"B = {self._B.get()}",
                               bg=PAL["on"] if self._B.get() else PAL["off"],fg="white",
                               font=("Segoe UI Bold",10),relief="flat",padx=16,pady=6,
                               command=lambda: self._toggle_btn("B"))
        self._b_btn.pack(side="left",padx=8)
        self._evaluate()

    def _toggle_btn(self,which):
        if which=="A": self._A.set(0 if self._A.get() else 1); self._a_btn.config(text=f"A = {self._A.get()}",bg=PAL["on"] if self._A.get() else PAL["off"])
        else: self._B.set(0 if self._B.get() else 1); self._b_btn.config(text=f"B = {self._B.get()}",bg=PAL["on"] if self._B.get() else PAL["off"])
        self._evaluate()

    def _evaluate(self):
        g=self._gate_var.get(); a=bool(self._A.get()); b=bool(self._B.get())
        out=int(GATES[g](a,b))
        col=PAL["on"] if out else PAL["off"]
        self._out_lbl.config(text=str(out),fg=col)
        self._out_txt.config(text="HIGH" if out else "LOW",fg=col)

    def _toggle(self,var): var.set(0 if var.get() else 1); self._evaluate()

    def _build_truth(self,nb):
        tab=tk.Frame(nb,bg=PAL["bg"]); nb.add(tab,text="  📊 Truth Tables  ")
        tk.Label(tab,text="Full Truth Tables — All Standard Gates",
                 fg=PAL["dim"],bg=PAL["bg"],font=("Segoe UI",10)).pack(pady=10)
        canvas=tk.Canvas(tab,bg=PAL["bg"],highlightthickness=0); canvas.pack(fill="both",expand=True,padx=16)
        sb=ttk.Scrollbar(tab,orient="vertical",command=canvas.yview); sb.pack(side="right",fill="y")
        canvas.configure(yscrollcommand=sb.set)
        frame=tk.Frame(canvas,bg=PAL["bg"]); canvas.create_window((0,0),window=frame,anchor="nw")
        col=0
        for gate, fn in GATES.items():
            gf=tk.Frame(frame,bg=PAL["card"],padx=12,pady=10,
                        highlightthickness=1,highlightbackground=PAL["accent"])
            gf.grid(row=0,column=col,padx=8,pady=8,sticky="n")
            col+=1
            tk.Label(gf,text=gate,fg=PAL["accent"],bg=PAL["card"],
                     font=("Segoe UI Bold",12)).grid(row=0,column=0,columnspan=4)
            heads=["A","B","OUT"] if gate!="NOT" else ["A","OUT"]
            for ci,h in enumerate(heads):
                tk.Label(gf,text=h,fg=PAL["dim"],bg=PAL["card"],
                         font=("Segoe UI Bold",9),width=4).grid(row=1,column=ci)
            row=2
            for a in (0,1):
                for b in (0,1):
                    out=int(fn(bool(a),bool(b)))
                    vals=[str(a),str(b),str(out)] if gate!="NOT" else [str(a),str(out)]
                    for ci,v in enumerate(vals):
                        c2=PAL["on"] if v=="1" else PAL["off"]
                        tk.Label(gf,text=v,fg=c2,bg=PAL["card"],
                                 font=("Cascadia Code",11),width=4).grid(row=row,column=ci)
                    row+=1
                    if gate=="NOT": break
        frame.update_idletasks(); canvas.configure(scrollregion=canvas.bbox("all"))

    def _build_adder(self,nb):
        tab=tk.Frame(nb,bg=PAL["bg"],padx=30,pady=20); nb.add(tab,text="  ➕ Adder  ")
        tk.Label(tab,text="Binary Adder Simulator",fg=PAL["accent"],
                 bg=PAL["bg"],font=("Segoe UI Bold",13)).pack(pady=(0,16))

        # 4-bit adder
        fr=tk.Frame(tab,bg=PAL["card"],padx=20,pady=16); fr.pack(fill="x")
        tk.Label(fr,text="4-bit Ripple Carry Adder",fg=PAL["dim"],bg=PAL["card"],
                 font=("Segoe UI",9,"bold")).grid(row=0,column=0,columnspan=9,sticky="w",pady=(0,8))
        self._bits_A=[tk.IntVar(value=0) for _ in range(4)]
        self._bits_B=[tk.IntVar(value=0) for _ in range(4)]
        for i in range(4):
            tk.Label(fr,text=f"A{3-i}",fg=PAL["dim"],bg=PAL["card"],
                     font=("Segoe UI",8)).grid(row=1,column=i*2)
            tk.Checkbutton(fr,variable=self._bits_A[i],bg=PAL["card"],
                           fg=PAL["on"],selectcolor=PAL["bg"],activebackground=PAL["card"],
                           command=self._adder_eval).grid(row=2,column=i*2)
            tk.Label(fr,text=f"B{3-i}",fg=PAL["dim"],bg=PAL["card"],
                     font=("Segoe UI",8)).grid(row=3,column=i*2)
            tk.Checkbutton(fr,variable=self._bits_B[i],bg=PAL["card"],
                           fg=PAL["accent"],selectcolor=PAL["bg"],activebackground=PAL["card"],
                           command=self._adder_eval).grid(row=4,column=i*2)
        self._adder_out=tk.Label(fr,text="A + B = ?",fg=PAL["on"],bg=PAL["card"],
                                  font=("Cascadia Code",14))
        self._adder_out.grid(row=5,column=0,columnspan=9,pady=10)

    def _adder_eval(self):
        A=sum(b.get()<<(3-i) for i,b in enumerate(self._bits_A))
        B=sum(b.get()<<(3-i) for i,b in enumerate(self._bits_B))
        S=A+B; carry=1 if S>15 else 0; S_4bit=S&15
        self._adder_out.config(text=f"{A} + {B} = {S_4bit}  (carry={carry})  BIN: {bin(S_4bit)}")

    def _build_combinational(self,nb):
        tab=tk.Frame(nb,bg=PAL["bg"],padx=24,pady=16); nb.add(tab,text="  🔗 Circuits  ")
        tk.Label(tab,text="Combinational Logic Circuits",fg=PAL["accent"],
                 bg=PAL["bg"],font=("Segoe UI Bold",13)).pack(pady=(0,12))
        circuits=[
            ("2:1 Multiplexer","sel,A,B → (A if sel=0 else B)","MUX"),
            ("1:2 Demultiplexer","sel,D → Y0=D if sel=0 else Y1=D","DEMUX"),
            ("Decoder 2:4","A,B → 4 output lines","DEC24"),
            ("Encoder 4:2","4 inputs → A,B priority encoding","ENC42"),
            ("Full Adder","A+B+Cin → Sum, Cout","FA"),
            ("Half Adder","A+B → Sum, Carry","HA"),
        ]
        for name,desc,tag in circuits:
            card=tk.Frame(tab,bg=PAL["card"],padx=16,pady=10,
                          highlightthickness=1,highlightbackground=PAL["border"])
            card.pack(fill="x",pady=4)
            tk.Label(card,text=name,fg=PAL["accent"],bg=PAL["card"],
                     font=("Segoe UI Bold",10)).pack(anchor="w")
            tk.Label(card,text=desc,fg=PAL["dim"],bg=PAL["card"],
                     font=("Segoe UI",9)).pack(anchor="w")
            tk.Button(card,text="SIMULATE →",bg=PAL["accent"],fg="white",
                      font=("Segoe UI",8),relief="flat",padx=12,pady=4,
                      command=lambda t=tag,n=name: self._simulate_circuit(t,n)).pack(anchor="e")

    def _simulate_circuit(self,tag,name):
        w=tk.Toplevel(self); w.title(f"Simulate — {name}")
        w.geometry("420x300"); w.configure(bg=PAL["bg"])
        out=tk.Text(w,bg=PAL["card"],fg=PAL["on"],font=("Cascadia Code",10),
                    borderwidth=0,padx=12,pady=12); out.pack(fill="both",expand=True,padx=16,pady=16)
        out.insert("end",f"CIRCUIT: {name}\n" + "─"*36 + "\n\n")
        if tag=="FA":
            for A in (0,1):
                for B in (0,1):
                    for Cin in (0,1):
                        S,Cout=full_adder(A,B,Cin)
                        out.insert("end",f"A={A} B={B} Cin={Cin}  →  Sum={S} Cout={Cout}\n")
        elif tag=="HA":
            for A in (0,1):
                for B in (0,1):
                    S,C=half_adder(A,B)
                    out.insert("end",f"A={A} B={B}  →  Sum={S} Carry={C}\n")
        elif tag=="MUX":
            for sel in (0,1):
                for A in (0,1):
                    for B in (0,1):
                        Y=A if sel==0 else B
                        out.insert("end",f"sel={sel} A={A} B={B}  →  Y={Y}\n")
        elif tag=="DEC24":
            for A in (0,1):
                for B in (0,1):
                    n=A*2+B; Y=[0,0,0,0]; Y[n]=1
                    out.insert("end",f"A={A} B={B}  →  Y3..0={Y[3]}{Y[2]}{Y[1]}{Y[0]}\n")
        else:
            out.insert("end","Simulation output for this circuit coming soon!\n")

def launch(kernel=None):
    LogicSimulator(kernel).mainloop()

if __name__=="__main__":
    launch()
