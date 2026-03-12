import tkinter as tk
from tkinter import scrolledtext
from .base_page import SigmaPage
from .styles import PAL

class AGGuidePage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "ANTIGRAVITY TOOLS GUIDE", "Comprehensive Manual \u0026 Feature Index")
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        txt = scrolledtext.ScrolledText(main_panel, bg=PAL["card"], fg=PAL["text"], font=("Consolas", 10), wrap="word", padx=10, pady=10)
        txt.pack(fill="both", expand=True)
        
        # Load the markdown content
        try:
             import os
             # Unified path discovery
             guide_path = os.path.join(os.getcwd(), "docs", "ANTIGRAVITY_TOOLS_GUIDE.md")
             if os.path.exists(guide_path):
                 with open(guide_path, "r", encoding="utf-8") as f:
                     content = f.read()
                     txt.insert("1.0", content)
             else:
                 txt.insert("1.0", "Antigravity Software Guide\n\nGuide file not found. All tools are accessible from the Enterprise Hub.")
        except Exception as e:
             txt.insert("1.0", f"Error loading guide: {e}")
        
        txt.configure(state="disabled")
