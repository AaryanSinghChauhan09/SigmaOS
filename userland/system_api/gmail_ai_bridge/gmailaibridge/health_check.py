# Generated method: GmailAIBridge.health_check
import os
import json
import time

class GmailAIBridge:
    def health_check(self) -> str:
        s = self.stats
        return f"OK — Gmail Bridge: {(self.current_user['email'] if self.current_user else 'None')} | Triage: {s['emails_triaged']} | Saved: {s['minutes_saved']}m"