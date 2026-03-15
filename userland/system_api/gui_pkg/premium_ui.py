import tkinter as tk
from .styles import PAL, FONT_BOLD, FONT_SMALL

class PremiumUIMixin:
    """Advanced UI components for an Above-Industry level experience."""
    
    def _premium_card(self, parent, title="", subtitle="", icon="💎", glass=True):
        """Creates a high-end card with subtle gradients and drop shadows (simulated)."""
        bg_color = PAL["bg2"] if glass else PAL["card"]
        border_color = PAL["accent"] if glass else PAL["border"]
        
        # Outer container for 'shadow' effect
        shadow = tk.Frame(parent, bg="#000000", padx=1, pady=1)
        
        # Main container
        container = tk.Frame(shadow, bg=bg_color, padx=15, pady=15)
        container.pack(fill="both", expand=True)
        container.master = shadow
        
        if title:
            header = tk.Frame(container, bg=bg_color)
            header.pack(fill="x", pady=(0, 5))
            
            tk.Label(header, text=icon, font=("Segoe UI", 12), fg=PAL["cyan"], bg=bg_color).pack(side="left", padx=(0, 10))
            
            title_fr = tk.Frame(header, bg=bg_color)
            title_fr.pack(side="left", fill="x")
            
            tk.Label(title_fr, text=title.upper(), font=FONT_BOLD, fg=PAL["text"], bg=bg_color).pack(anchor="w")
            if subtitle:
                tk.Label(title_fr, text=subtitle, font=("Segoe UI", 8), fg=PAL["dim"], bg=bg_color).pack(anchor="w")
                
            tk.Frame(container, bg=PAL["bg3"], height=1).pack(fill="x", pady=(10, 15))
            
        return container

    def _pulsing_button(self, parent, text, command, color=None):
        """A button that has a subtle 'glow' or 'pulse' on hover."""
        btn_color = color or PAL["accent"]
        
        btn = tk.Button(
            parent, text=text, command=command,
            bg=btn_color, fg="white", font=FONT_BOLD,
            activebackground=PAL["accent2"], activeforeground="white",
            relief="flat", bd=0, padx=20, pady=8
        )
        
        def on_enter(e):
            btn.config(bg=PAL["accent2"])
            # In a real premium app, we'd start a pulse animation here
            
        def on_leave(e):
            btn.config(bg=btn_color)
            
        btn.bind("<Enter>", on_enter)
        btn.bind("<Leave>", on_leave)
        return btn

    def _frosted_entry(self, parent, placeholder="Enter command..."):
        """A sleek entry field with a 'glass' look."""
        container = tk.Frame(parent, bg=PAL["border"], padx=1, pady=1)
        entry = tk.Entry(
            container, bg=PAL["bg3"], fg=PAL["text"],
            font=("Segoe UI", 10), insertbackground=PAL["cyan"],
            relief="flat", bd=8
        )
        entry.pack(fill="x")
        
        def on_focus_in(e):
            container.config(bg=PAL["cyan"])
            if entry.get() == placeholder:
                entry.delete(0, "end")
                entry.config(fg=PAL["text"])

        def on_focus_out(e):
            container.config(bg=PAL["border"])
            if not entry.get():
                entry.insert(0, placeholder)
                entry.config(fg=PAL["dim"])

        entry.insert(0, placeholder)
        entry.bind("<FocusIn>", on_focus_in)
        entry.bind("<FocusOut>", on_focus_out)
        
        entry.container = container
        return entry
