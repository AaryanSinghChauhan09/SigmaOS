import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD

class GmailAIPage(SigmaPage):
    def __init__(self, parent, gui):
        super().__init__(parent, gui, "GMAIL AI BRIDGE", "Seamlessly Connect Sovereign OS with Google Workspace AI")
        self._build_content()
        
    def _build_content(self):
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        # Left Panel (Authentication)
        auth_panel_container = self._card(main_panel, "Workspace Credentials")
        auth_panel_container.master.pack(side="left", fill="y", expand=False, padx=(0,10))
        auth_panel_container.master.configure(width=300)
        auth_panel_container.master.pack_propagate(False)
        auth_panel = auth_panel_container
        
        tk.Label(auth_panel, text="Email Address", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w", pady=(10, 0))
        email_entry = ttk.Entry(auth_panel, width=30)
        email_entry.pack(fill="x", pady=5)
        
        tk.Label(auth_panel, text="App Password / Token", font=FONT_SMALL, fg=PAL["dim"], bg=PAL["card"]).pack(anchor="w", pady=(5, 0))
        pass_entry = ttk.Entry(auth_panel, show="*", width=30)
        pass_entry.pack(fill="x", pady=5)
        
        status_lbl = tk.Label(auth_panel, text="Not Authenticated", fg=PAL["red"], bg=PAL["card"])
        status_lbl.pack(pady=10)
        
        def _login():
            email = email_entry.get()
            pwd = pass_entry.get()
            if not email or not pwd: return
            
            if not hasattr(self.kernel, 'gmail_ai'):
                from gmail_ai_bridge import GmailAIBridge
                self.kernel.gmail_ai = GmailAIBridge(self.kernel)
                
            res = self.kernel.gmail_ai.login(email, pwd)
            status_lbl.configure(text=f"Logged in as: {res['profile']['email']}", fg=PAL["green"])
            email_entry.delete(0, tk.END)
            pass_entry.delete(0, tk.END)
            self._notify("GMAIL AI", f"Connected to {email}!", "OK")
            _update_status()
            
        ttk.Button(auth_panel, text="Authenticate Session", command=_login).pack(fill="x", pady=5)
        
        tk.Label(auth_panel, text="Connected Services:\n• Gemini 1.5 Pro\n• Gemini Flash\n• Workspace Assistants", 
                 font=FONT_SMALL, fg=PAL["cyan"], bg=PAL["card"], justify="left").pack(anchor="w", pady=20)
        
        # Right Panel (AI Interaction)
        chat_panel_container = self._card(main_panel, "Sovereign AI Terminal")
        chat_panel_container.master.pack(side="right", fill="both", expand=True)
        chat_panel = chat_panel_container
        
        chat_log = self._console(chat_panel, height=20)
        chat_log.pack(fill="both", expand=True, pady=10)
        self._log(chat_log, "SYSTEM: Awaiting Gmail Authentication to initialize AI pipeline...", "INFO")
        
        input_fr = tk.Frame(chat_panel, bg=PAL["card"])
        input_fr.pack(fill="x")
        
        prompt_var = tk.StringVar()
        prompt_entry = ttk.Entry(input_fr, textvariable=prompt_var)
        prompt_entry.pack(side="left", fill="x", expand=True, padx=(0,5))
        
        def _query_ai(e=None):
            if not hasattr(self.kernel, 'gmail_ai') or not self.kernel.gmail_ai.active_profile:
                self._notify("ERROR", "Please connect a Gmail account first.", "ERR")
                return
                
            prompt = prompt_var.get()
            if not prompt: return
            
            self._log(chat_log, f"You: {prompt}", "OK")
            prompt_var.set("")
            
            status_lbl.update()
            res = self.kernel.gmail_ai.query_gemini(prompt)
            if res.get("status") == "SUCCESS":
                self._log(chat_log, f"AI [{res['model']}]:\n{res['response']}\n(Latency: {res['latency_ms']}ms)", "TRACE")
            else:
                self._log(chat_log, f"AI Error: {res.get('response')}", "ERR")
                
        prompt_entry.bind("<Return>", _query_ai)
        ttk.Button(input_fr, text="Send to Cloud AI", command=_query_ai).pack(side="right")
        
        def _update_status():
            if hasattr(self.kernel, 'gmail_ai') and self.kernel.gmail_ai.active_profile:
                status_lbl.configure(text=f"Logged in as: {self.kernel.gmail_ai.active_profile}", fg=PAL["green"])
                self._log(chat_log, f"SYSTEM: Connection established with {self.kernel.gmail_ai.active_profile}!", "OK")
        
        _update_status()
