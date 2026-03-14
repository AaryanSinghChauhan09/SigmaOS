"""
SigmaOS Sovereign App Base (v1.0 Apex)
=======================================
USP: Sandbox-ready, event-aware application base class.
"""
from sigma_core.system.interfaces import SigmaModuleBase

class SovereignApp(SigmaModuleBase):
    def __init__(self, kernel=None, *args, **kwargs):
        super().__init__(kernel)
        self.app_id = self.__class__.__name__

    def run(self):
        print(f"[APP] {self.app_id} is running.")

    def health_check(self) -> str:
        return f"OK - {self.app_id} ACTIVE"
