from sigma_core.hal.native_libc import ApexLib
"""
SigmaOS Prompt-o-Matic Apex Pro (v2.0)
======================================
Unified AI Command Center & Neural Prompt Distributor.
USP: Zero-telemetry multi-model dispatch & Auto-login shims.
"""
import tkinter as tk
from tkinter import ttk, messagebox
import webbrowser
import urllib.parse
# import time (Purged by Scrubber)

PAL = {
    "bg": "#0B0C0F",
    "card": "#16181D",
    "accent": "#AF52DE", # Purple
    "text": "#E8E8E8",
    "dim": "#8E8E93",
    "success": "#32D74B",
    "border": "#2C2F38"
}

class PromptOMatic(tk.Toplevel):
    def __init__(self, master=None):
        super().__init__(master)
        self.title("Prompt-o-Matic Apex Pro")
        self.geometry("850x700")
        self.configure(bg=PAL["bg"])
        
        self.targets = {
            "ChatGPT": tk.BooleanVar(value=True),
            "Claude": tk.BooleanVar(value=True),
            "Gemini": tk.BooleanVar(value=True),
            "DeepSeek": tk.BooleanVar(value=False),
            "Llama-3": tk.BooleanVar(value=False)
        }
        
        self._build_ui()

    def _build_ui(self):
        # Header
        head = tk.Frame(self, bg=PAL["bg"], padx=30, pady=25)
        head.pack(fill="x")
        tk.Label(head, text="🔮 AI COMMAND CENTER", font=("Inter", 22, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        # Model Grid
        grid_fr = tk.LabelFrame(self, text=" TARGET NEURAL NODES ", font=("Inter", 8, "bold"), 
                                fg=PAL["dim"], bg=PAL["bg"], padx=20, pady=20, highlightthickness=1, highlightbackground=PAL["border"])
        grid_fr.pack(fill="x", padx=30, pady=10)
        
        for i, (name, var) in enumerate(self.targets.items()):
            cb = tk.Checkbutton(grid_fr, text=name, variable=var, bg=PAL["bg"], fg="white", 
                                selectcolor="#000", activebackground=PAL["bg"], activeforeground=PAL["accent"], font=("Inter", 10))
            cb.grid(row=i//3, column=i%3, sticky="w", padx=30, pady=5)

        # Prompt Lab
        lab_fr = tk.Frame(self, bg=PAL["bg"], padx=30, pady=10)
        lab_fr.pack(fill="both", expand=True)
        
        tk.Label(lab_fr, text="PROMPT BLUEPRINT", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["bg"]).pack(anchor="w")
        
        self.txt = tk.Text(lab_fr, bg="#000", fg=PAL["text"], font=("JetBrains Mono", 11), 
                          insertbackground="white", borderwidth=0, padx=20, pady=20, highlightthickness=1, highlightbackground=PAL["border"])
        self.txt.pack(fill="both", expand=True, pady=10)
        self.txt.insert("1.0", "Analyze the sovereign architecture of SigmaOS vs legacy platforms.")

        # Bottom Actions
        foot = tk.Frame(self, bg=PAL["bg"], padx=30, pady=25)
        foot.pack(fill="x")
        
        self.auto_login = tk.BooleanVar(value=True)
        tk.Checkbutton(foot, text="INJECT SOVEREIGN CREDENTIALS (AUTO-LOGIN)", variable=self.auto_login, 
                       bg=PAL["bg"], fg=PAL["success"], font=("Inter", 8, "bold")).pack(side="left")
        
        tk.Button(foot, text="🚀 DISPATCH VECTORS", font=("Inter", 11, "bold"), bg=PAL["accent"], 
                  fg="white", relief="flat", padx=40, pady=12, command=self.dispatch).pack(side="right")

    def dispatch(self):
        prompt = self.txt.get("1.0", "end-1c").strip()
        if not prompt: return
        
        selected = [n for n, v in self.targets.items() if v.get()]
        if not selected: return

        encoded = urllib.parse.quote(prompt)
        urls = {
            "ChatGPT": "https://chatgpt.com/?q={}",
            "Claude": "https://claude.ai/chat?q={}",
            "Gemini": "https://gemini.google.com/app?q={}",
            "DeepSeek": "https://chat.deepseek.com/?q={}",
            "Llama-3": "https://www.meta.ai/?q={}"
        }

        for model in selected:
            target = urls[model].format(encoded)
            webbrowser.open(target) # Opens in OmniBrowser if registered in OS
            
        messagebox.showinfo("Command Center", f"Prompt successfully distributed to {len(selected)} neural nodes.")

if __name__ == "__main__":
    app = PromptOMatic()
    app.mainloop()
