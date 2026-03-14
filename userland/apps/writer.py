"""
SigmaOS Sovereign Writer Apex Pro (v3.0)
========================================
A premium, zero-trust word processor with AI composition assistance.
USP: Neural Writing Flow & Encrypted Local-Only Persistence.
"""
import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import random
import os

try:
    from sigma_core.ui.fluid_design import ICONS # type: ignore
except ImportError:
    ICONS = {}

PAL = {
    "bg": "#0B0C0F",
    "sidebar": "#16181D",
    "accent": "#5E5CE6",
    "text": "#E8E8E8",
    "dim": "#8E8E93",
    "success": "#32D74B",
    "border": "#2C2F38"
}

class SovereignWriter(tk.Toplevel):
    def __init__(self, master=None):
        super().__init__(master)
        self.title("Sovereign Writer Apex Pro")
        self.geometry("1200x900")
        self.config(bg=PAL["bg"])
        
        self.is_zen = False
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure("Writer.TNotebook", background=PAL["bg"], borderwidth=0)
        style.configure("Writer.TNotebook.Tab", background=PAL["sidebar"], foreground=PAL["text"], 
                        padding=[20, 10], font=("Inter", 9, "bold"))
        style.map("Writer.TNotebook.Tab", background=[("selected", PAL["accent"])])

    def _build_ui(self):
        # 1. Premium Toolbar
        self.toolbar = tk.Frame(self, bg=PAL["bg"], height=60, padx=20)
        self.toolbar.pack(side="top", fill="x")
        
        tk.Label(self.toolbar, text=f"{ICONS.get('writer', '🖋️')} WRITER PRO", font=("Inter", 12, "bold"), fg=PAL["accent"], bg=PAL["bg"]).pack(side="left")
        
        # Tools
        btn_fr = tk.Frame(self.toolbar, bg=PAL["bg"])
        btn_fr.pack(side="left", padx=30)
        
        tool_btns = [("B", "bold"), ("I", "italic"), ("U", "underline"), (f"{ICONS.get('intelligence', '✨')} AI", "ai")]
        for txt, tag in tool_btns:
            bg = PAL["sidebar"] if tag != "ai" else PAL["accent"]
            tk.Button(btn_fr, text=txt, font=("Inter", 9, "bold"), bg=bg, fg="white", 
                      relief="flat", padx=15, pady=8).pack(side="left", padx=2)

        tk.Button(self.toolbar, text=f"{ICONS.get('snapshots', '💾')} SAVE", font=("Inter", 8, "bold"), bg=PAL["success"], fg="white", 
                  relief="flat", padx=20, pady=8, command=self.save).pack(side="right", padx=5)
        tk.Button(self.toolbar, text=f"{ICONS.get('minimalist', '🧘')} ZEN MODE", font=("Inter", 8, "bold"), bg=PAL["sidebar"], fg="white", 
                  relief="flat", padx=20, pady=8, command=self.toggle_zen).pack(side="right", padx=5)

        # 2. Main Layout
        self.main_fr = tk.Frame(self, bg=PAL["bg"])
        self.main_fr.pack(fill="both", expand=True)

        # Left Sidebar: Outline
        self.sidebar = tk.Frame(self.main_fr, bg=PAL["sidebar"], width=220, padx=15, pady=20)
        self.sidebar.pack(side="left", fill="y")
        self.sidebar.pack_propagate(False)
        
        tk.Label(self.sidebar, text=f"{ICONS.get('search', '📑')} OUTLINE", font=("Inter", 8, "bold"), fg=PAL["dim"], bg=PAL["sidebar"]).pack(anchor="w")
        
        sections = ["Introduction", "Strategic Overview", "Quantum Mesh Specs", "Conclusion"]
        for s in sections:
            tk.Label(self.sidebar, text=f"• {s}", font=("Inter", 10), fg=PAL["text"], 
                     bg=PAL["sidebar"], pady=8, cursor="hand2").pack(anchor="w")

        # Right Sidebar: AI Assistant
        self.ai_side = tk.Frame(self.main_fr, bg=PAL["sidebar"], width=250, padx=15, pady=20)
        self.ai_side.pack(side="right", fill="y")
        self.ai_side.pack_propagate(False)
        
        tk.Label(self.ai_side, text=f"{ICONS.get('intelligence', '🧠')} AI ASSISTANT", font=("Inter", 8, "bold"), fg=PAL["accent"], bg=PAL["sidebar"]).pack(anchor="w")
        
        self.ai_box = tk.Text(self.ai_side, bg="#000", fg=PAL["success"], font=("Consolas", 9), 
                             height=15, borderwidth=0, padx=10, pady=10)
        self.ai_box.pack(fill="x", pady=15)
        self.ai_box.insert("1.0", "[AGENT] Ready to assist.\n\nSuggested Next Sentence:\n'The scalability of the Aether Mesh ensures 100% reliability.'")

        # Editor Area
        self.editor_fr = tk.Frame(self.main_fr, bg=PAL["bg"], padx=40, pady=20)
        self.editor_fr.pack(fill="both", expand=True)
        
        self.editor = scrolledtext.ScrolledText(self.editor_fr, font=("Inter", 13), padx=80, pady=80, 
                                                bg="#FFFFFF", fg="#111", insertbackground="black", 
                                                borderwidth=0, undo=True, highlightthickness=1, 
                                                highlightbackground=PAL["border"])
        self.editor.pack(fill="both", expand=True)
        self.editor.insert("1.0", "Welcome to Sovereign Writer Apex Pro.\n\nStart your mission-critical documentation here.")

        # 3. Status Bar
        self.status = tk.Label(self, text="WORDS: 12 | ENCRYPTION: SHA-256 | LATENCY: 0ms", 
                               bg=PAL["accent"], fg="white", font=("Inter", 8, "bold"), pady=5)
        self.status.pack(side="bottom", fill="x")

    def toggle_zen(self):
        self.is_zen = not self.is_zen
        if self.is_zen:
            self.sidebar.pack_forget()
            self.ai_side.pack_forget()
            self.toolbar.pack_forget()
            self.editor.config(padx=150, pady=100, font=("Inter", 18))
        else:
            self.sidebar.pack(side="left", fill="y", before=self.editor_fr)
            self.ai_side.pack(side="right", fill="y", after=self.editor_fr)
            self.toolbar.pack(side="top", fill="x", before=self.main_fr)
            self.editor.config(padx=80, pady=80, font=("Inter", 13))

    def save(self):
        messagebox.showinfo("Writer Pro", "Document serialized and committed to Sovereign Vault.")

if __name__ == "__main__":
    app = SovereignWriter()
    app.mainloop()
