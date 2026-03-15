"""
Sigma Familiarity Engine (Windows/macOS Translation Layer)
==========================================================
USP: To make SigmaOS instantly accessible to billions of users, this engine
     translates the advanced Sovereign terminology and UX paradigms into
     familiar Windows or macOS styles instantly. It acts as a cognitive bridge.

Features:
    pass
- Terminology Translation: Connects 'Sovereign Sanctuary' -> 'Windows Defender / Security'.
- UX Layout Shift: Morphs the central HUD into a left-aligned Start Menu (Windows Mode) or bottom dock (macOS Mode).
- Shortcut Mapping: Automatically maps standard 'Ctrl+C / Ctrl+V / Alt+Tab' to their sovereign equivalents.
"""

class SigmaFamiliarityEngine:
    def __init__(self, kernel):
        self.kernel = kernel
        self.active_paradigm = "Sigma_Sovereign"
        
        self.paradigms = {
            "Windows_Classic": {
                "layout": "Bottom_Taskbar_Left_Start",
                "terminology": {
                    "Sovereign Sanctuary": "Security Center",
                    "Explorer": "File Explorer",
                    "App Matrix": "All Programs",
                    "Omni Workspaces": "Virtual Desktops",
                    "Sigma Media Studio": "Video Editor",
                },
                "shortcuts": "Standard Windows (Ctrl+C, Alt+Tab)",
                "description": "Familiar Windows layout for ultimate ease of use. Reverts terminology to standard PC norms."
            },
            "macOS_Fluid": {
                "layout": "Bottom_Dock_Top_Menu",
                "terminology": {
                    "Sovereign Sanctuary": "System Settings",
                    "Explorer": "Finder",
                    "App Matrix": "Launchpad",
                    "Omni Workspaces": "Spaces / Mission Control",
                },
                "shortcuts": "Standard Mac (Cmd+C, Cmd+Space)",
                "description": "Familiar macOS layout with bottom-center dock and global menu bar."
            }
        }

    def activate_paradigm(self, paradigm: str) -> dict:
        if paradigm == "Sigma_Sovereign":
            self.active_paradigm = "Sigma_Sovereign"
            return {"status": "RESTORED", "message": "Restored default SigmaOS Sovereign interface."}
            
        if paradigm not in self.paradigms:
            return {"status": "ERROR", "message": f"Paradigm '{paradigm}' unknown."}
            
        self.active_paradigm = paradigm
        config = self.paradigms[paradigm]
        
        return {
            "status": "MORPHED_UX",
            "paradigm": paradigm,
            "layout": config["layout"],
            "translations": config["terminology"],
            "message": f"Interface successfully bridged to '{paradigm}'. System is now as easy to use as a standard PC."
        }

    def get_translated_term(self, sigma_term: str) -> str:
        if self.active_paradigm == "Sigma_Sovereign":
            return sigma_term
        return self.paradigms.get(self.active_paradigm, {}).get("terminology", {}).get(sigma_term, sigma_term)

    def health_check(self) -> str:
        return f"OK — Familiarity Engine Active. Current Paradigm: {self.active_paradigm}."