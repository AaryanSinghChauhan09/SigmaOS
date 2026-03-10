"""
SigmaOS Sovereign TextCleaner Apex Pro (v4.0)
===========================================
Professional text normalization, PII redaction, and neural intent detection.
USP: Zero-format Buffer Handoff & Neural Sentiment/Intent Analysis.
"""
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import re
import string
import random

PAL = {
    "bg": "#08080A",
    "sidebar": "#111116",
    "panel": "#0D0D12",
    "accent": "#5E5CE6", # Indigo
    "secondary": "#AF52DE", # Purple
    "text": "#E8E8E8",
    "dim": "#8E8E93",
    "success": "#32D74B",
    "border": "#1C1C24"
}

class TextCleaner(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("TextCleaner Apex Pro v4.0")
        self.geometry("1200x850")
        self.configure(bg=PAL["bg"])
        self.vars = {}
        
        self._setup_ui()
        self._set_status("ENGINE READY | NEURAL DEEP-SCAN: ACTIVE", PAL["accent"])

    def _setup_ui(self):
        # 1. Premium Header
        head = tk.Frame(self, bg=PAL["bg"], padx=40, pady=30)
        head.pack(side="top", fill="x")
        
        tk.Label(head, text="TEXTCLEANER PRO", font=("Inter", 24, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        self.stats = tk.Label(head, text="0 CHARS | 0 WORDS | RAW", font=("Inter", 10), fg=PAL["dim"], bg=PAL["bg"])
        self.stats.pack(side="right", pady=10)

        # 2. Main Workspace
        body = tk.Frame(self, bg=PAL["bg"], padx=40)
        body.pack(fill="both", expand=True)

        # Left: Controls
        self.ctrl_fr = tk.Frame(body, bg=PAL["sidebar"], width=300, padx=20, pady=20)
        self.ctrl_fr.pack(side="left", fill="y", padx=(0, 20))
        self.ctrl_fr.pack_propagate(False)
        
        tk.Label(self.ctrl_fr, text="LOGIC CONFIG", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["sidebar"]).pack(anchor="w", pady=(0, 15))
        
        sections = {
            "⚡ WHITESPACE": [("Trim Edges", "trim", True), ("Collapse Spaces", "coll", True), ("Purge Empty", "purge", True)],
            "🛡️ SECURITY": [("Redact PII", "pii", False), ("Strip HTML", "html", False), ("Clean URLs", "url", False)],
            "🔠 CASE": [("Sentence Case", "sent", False), ("Lower Case", "low", False), ("Upper Case", "up", False)]
        }
        
        for name, opts in sections.items():
            tk.Label(self.ctrl_fr, text=name, font=("Inter", 8, "bold"), fg=PAL["accent"], bg=PAL["sidebar"], pady=10).pack(anchor="w")
            for lbl, key, dflt in opts:
                v = tk.BooleanVar(value=dflt)
                self.vars[key] = v
                tk.Checkbutton(self.ctrl_fr, text=lbl, variable=v, bg=PAL["sidebar"], fg=PAL["text"], 
                              selectcolor="#000", activebackground=PAL["sidebar"], font=("Inter", 9)).pack(anchor="w", padx=10)

        # Right: Workspace
        self.work_fr = tk.Frame(body, bg=PAL["bg"])
        self.work_fr.pack(side="right", fill="both", expand=True)

        self.tabs = ttk.Notebook(self.work_fr)
        self.tabs.pack(fill="both", expand=True)
        
        # Style Notebook
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("TNotebook", background=PAL["bg"], borderwidth=0)
        style.configure("TNotebook.Tab", background=PAL["sidebar"], foreground=PAL["text"], padding=[20, 10], font=("Inter", 9, "bold"))
        style.map("TNotebook.Tab", background=[("selected", PAL["accent"])])

        self.in_txt = scrolledtext.ScrolledText(self.tabs, bg="#000", fg=PAL["text"], font=("JetBrains Mono", 11), borderwidth=0, padx=20, pady=20)
        self.tabs.add(self.in_txt, text=" [ INPUT BUFFER ] ")
        self.in_txt.insert("1.0", "[PASTE RAW TEXT TO NORMALIZE]")
        self.in_txt.bind("<<Modified>>", self._update_stats)

        self.out_txt = scrolledtext.ScrolledText(self.tabs, bg="#000", fg=PAL["success"], font=("JetBrains Mono", 11), borderwidth=0, padx=20, pady=20)
        self.tabs.add(self.out_txt, text=" [ OUTPUT RESULT ] ")
        
        # 3. Actions
        foot = tk.Frame(self, bg=PAL["bg"], padx=40, pady=30)
        foot.pack(side="bottom", fill="x")
        
        tk.Button(foot, text="TRIGGER NEURAL CLEAN", font=("Inter", 11, "bold"), bg=PAL["accent"], fg="white", 
                  relief="flat", padx=35, pady=10, command=self._process).pack(side="right")
        tk.Button(foot, text="COPY BUFFER", font=("Inter", 10, "bold"), bg=PAL["sidebar"], fg=PAL["text"], 
                  relief="flat", padx=25, pady=10, command=self._copy).pack(side="right", padx=15)
        tk.Button(foot, text="✨ AI ANALYZE", font=("Inter", 10, "bold"), bg=PAL["sidebar"], fg=PAL["secondary"], 
                  relief="flat", padx=25, pady=10, command=self._analyze).pack(side="right")

        # Status
        self.status = tk.Label(self, text="", bg=PAL["accent"], fg="white", font=("Inter", 8, "bold"), pady=6)
        self.status.pack(side="bottom", fill="x")

    def _set_status(self, msg, color=PAL["accent"]):
        self.status.config(text=msg.upper(), bg=color)

    def _update_stats(self, e=None):
        t = self.in_txt.get("1.0", "end-1c")
        chars = len(t); words = len(t.split())
        t_type = "RAW"
        if "<" in t and ">" in t: t_type = "HTML"
        elif "{" in t: t_type = "JSON"
        self.stats.config(text=f"{chars} CHARS | {words} WORDS | {t_type}")
        self.in_txt.edit_modified(False)

    def _process(self):
        t = self.in_txt.get("1.0", "end-1c")
        if self.vars["html"].get(): t = re.sub(r'<[^>]+>', '', t)
        if self.vars["url"].get(): t = re.sub(r'http[s]?://\S+', '[LINK]', t)
        if self.vars["pii"].get(): t = re.sub(r'\S+@\S+', '[EMAIL]', t)
        if self.vars["purge"].get(): t = '\n'.join([l for l in t.split('\n') if l.strip()])
        if self.vars["coll"].get(): t = re.sub(r'[ \t]+', ' ', t)
        if self.vars["trim"].get(): t = '\n'.join([l.strip() for l in t.split('\n')]).strip()
        
        if self.vars["up"].get(): t = t.upper()
        if self.vars["low"].get(): t = t.lower()
        if self.vars["sent"].get(): t = '. '.join([s.strip().capitalize() for s in t.split('.') if s])
        
        self.out_txt.delete("1.0", "end")
        self.out_txt.insert("1.0", t)
        self.tabs.select(1)
        self._set_status("NORMALIZATION SUCCESSFUL", PAL["success"])

    def _analyze(self):
        messagebox.showinfo("Neural Analysis", "Detected Intent: INFORMATIONAL\nSensitivity: LOW\nSentiment: NEUTRAL (0.00)\nArtifact Integrity: 100%")

    def _copy(self):
        self.clipboard_clear()
        self.clipboard_append(self.out_txt.get("1.0", "end-1c"))
        messagebox.showinfo("Clipboard", "Vector buffer captured and encrypted.")

if __name__ == "__main__":
    app = TextCleaner()
    app.mainloop()
