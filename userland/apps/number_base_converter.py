"""
SigmaOS Number Base Converter v1.0
DEcimal / HEXadecimal / BINary / OCtAL + ASCII table
100% stdlib, zero 3rd-party deps.
"""
import tkinter as tk
from tkinter import ttk

PAL={"bg":"#0D0F18","panel":"#13162A","card":"#1A1E30","accent":"#22C55E",
     "text":"#E8E8F0","dim":"#9090A0","border":"#2A2D45","danger":"#FF4D4D"}

class NumberBaseConverter(tk.Tk):
    def __init__(self, kernel=None):
        super().__init__()
        self.title("SigmaOS Number Base Converter")
        self.geometry("640x560"); self.configure(bg=PAL["bg"]); self.resizable(False,False)
        self._build()

    def _build(self):
        tk.Frame(self, bg=PAL["panel"], height=50).pack(fill="x")
        hdr = self.children[list(self.children)[-1]]
        hdr.pack_propagate(False)
        tk.Label(hdr, text="⬡ NUMBER BASE CONVERTER", fg=PAL["accent"],
                 bg=PAL["panel"], font=("Segoe UI Bold",13)).pack(side="left",padx=18,pady=10)

        body = tk.Frame(self, bg=PAL["bg"], padx=24, pady=18)
        body.pack(fill="both", expand=True)

        self._vars = {}
        self._entries = {}
        bases = [("Decimal (Base 10)","dec",10),("Binary (Base 2)","bin",2),
                 ("Octal (Base 8)","oct",8),("Hexadecimal (Base 16)","hex",16)]

        for i,(lbl,key,base) in enumerate(bases):
            tk.Label(body, text=lbl, fg=PAL["dim"], bg=PAL["bg"],
                     font=("Segoe UI",9)).grid(row=i,column=0,sticky="w",pady=6)
            v = tk.StringVar()
            e = tk.Entry(body, textvariable=v, bg=PAL["card"], fg="white",
                         font=("Cascadia Code",13), insertbackground="white",
                         relief="flat", highlightthickness=1, highlightbackground=PAL["border"],
                         width=28)
            e.grid(row=i,column=1,padx=10,pady=6,sticky="w")
            self._vars[key] = (v, base)
            self._entries[key] = e
            v.trace_add("write", lambda *_, k=key: self._on_change(k))

        # Bit length
        tk.Label(body, text="Bit width", fg=PAL["dim"], bg=PAL["bg"],
                 font=("Segoe UI",9)).grid(row=4,column=0,sticky="w",pady=6)
        self._bits = tk.StringVar(value="32")
        ttk.Combobox(body, textvariable=self._bits, values=["8","16","32","64"],
                     state="readonly", width=8).grid(row=4,column=1,sticky="w",padx=10)

        # Math ops
        sep = tk.Frame(body, bg=PAL["border"], height=1)
        sep.grid(row=5,column=0,columnspan=2,sticky="ew",pady=12)

        ops_fr = tk.Frame(body, bg=PAL["bg"])
        ops_fr.grid(row=6,column=0,columnspan=2,sticky="w")
        tk.Label(ops_fr, text="Bitwise:", fg=PAL["dim"], bg=PAL["bg"],
                 font=("Segoe UI",9)).pack(side="left")
        for op in ("AND","OR","XOR","NOT","<<",">>"):
            tk.Button(ops_fr, text=op, bg=PAL["card"], fg=PAL["text"],
                      font=("Segoe UI",8), relief="flat", padx=8, pady=4,
                      command=lambda o=op: self._bitwise(o)).pack(side="left",padx=3)

        self._result = tk.Label(body, text="", fg=PAL["accent"], bg=PAL["bg"],
                                font=("Cascadia Code",12), wraplength=500, justify="left")
        self._result.grid(row=7,column=0,columnspan=2,sticky="w",pady=10)

        # ASCII table (mini)
        sep2 = tk.Frame(body, bg=PAL["border"], height=1)
        sep2.grid(row=8,column=0,columnspan=2,sticky="ew",pady=8)
        tk.Label(body, text="ASCII Quick Ref (32-127)", fg=PAL["dim"], bg=PAL["bg"],
                 font=("Segoe UI",8,"bold")).grid(row=9,column=0,columnspan=2,sticky="w")
        ascii_fr = tk.Frame(body, bg=PAL["card"])
        ascii_fr.grid(row=10,column=0,columnspan=2,sticky="w",pady=4)
        for i,code in enumerate(range(32,128)):
            ch=chr(code)
            tk.Label(ascii_fr, text=f"{code}={ch}", fg=PAL["dim"], bg=PAL["card"],
                     font=("Cascadia Code",7), width=6).grid(row=i//20,column=i%20)

    def _on_change(self, source_key):
        try:
            v_str, base = self._vars[source_key]
            val_str = v_str[0].get().strip()
            if not val_str: return
            n = int(val_str, base)
            for key, (v, b) in self._vars.items():
                if key == source_key: continue
                v[0].trace_remove("write", v[0].trace_info()[0][1]) if v[0].trace_info() else None
            for key, (v, b) in self._vars.items():
                if key == source_key: continue
                rep = (bin(n)[2:] if b==2 else oct(n)[2:] if b==8 else
                       hex(n)[2:].upper() if b==16 else str(n))
                try:
                    self._entries[key].delete(0,"end")
                    self._entries[key].insert(0, rep)
                except Exception: pass
        except (ValueError, tk.TclError): pass

    def _bitwise(self, op):
        try:
            a = int(self._vars["dec"][0][0].get())
            if op == "NOT":
                bits = int(self._bits.get())
                r = (~a) & ((1<<bits)-1)
                self._result.config(text=f"NOT {a} = {r}  (BIN: {bin(r)})")
            else:
                b_str = tk.simpledialog.askstring("Operand B","Enter second number (decimal):") if hasattr(tk,"simpledialog") else ""
                if not b_str: self._result.config(text="Enter B in Decimal field"); return
                b = int(b_str)
                ops={"AND":a&b,"OR":a|b,"XOR":a^b,"<<":(a<<b),">>":a>>b}
                r=ops[op]
                self._result.config(text=f"{a} {op} {b} = {r}  (BIN: {bin(r)}, HEX: {hex(r).upper()})")
        except Exception as ex:
            self._result.config(text=f"Error: {ex}")

def launch(kernel=None):
    NumberBaseConverter(kernel).mainloop()

if __name__=="__main__":
    launch()
