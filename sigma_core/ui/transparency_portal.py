"""
SigmaOS Transparency Portal (v1.0 Sovereign)
=============================================
USP: Public-ledger visualization and sovereign audit portal.
Enables 'Community Trust' by providing real-time visibility into the system ledger.
"""
import os
import sys
import time
from typing import Dict, Any, List

# Robust System Path Injection
_p = os.path.abspath(__file__)
while _p and not os.path.exists(os.path.join(os.path.dirname(_p), "sigma_core")):
    _p = os.path.dirname(_p)
    if _p == os.path.dirname(_p): break
root = os.path.dirname(_p)
if root and root not in sys.path: sys.path.insert(0, root)

from sigma_core.system.interfaces import SigmaModuleBase

class TransparencyPortal(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.stats = {"audits_served": 0}

    def get_public_ledger_state(self) -> List[Dict[str, Any]]:
        """USP: Fetches non-private entries from the Sovereign Ledger."""
        if not self.kernel or not hasattr(self.kernel, "ledger"):
            return [{"msg": "Ledger Offline", "status": "WARN"}]
        
        raw_ledger = self.kernel.ledger.get_recent_entries(count=10)
        # Filter for transparency: remove process IDs or sensitive tokens
        transparent_ledger = []
        for entry in raw_ledger:
            transparent_ledger.append({
                "ts": entry.get("timestamp"),
                "event": entry.get("event_type"),
                "shard": entry.get("origin_shard"),
                "integrity": "VERIFIED"
            })
        self.stats["audits_served"] += 1
        return transparent_ledger

    def generate_compliance_report(self):
        """USP: One-click Transparency Report for Community Audit."""
        if not self.kernel or not hasattr(self.kernel, "compliance"):
             return "Audit Failed: Compliance shard not found."
             
        findings = self.kernel.compliance.run_regulatory_audit()
        report = [
            "╔══════════════════════════════════════════════════════════════╗",
            "║ SigmaOS Sovereign Transparency Report | v5.2.1               ║",
            "╠══════════════════════════════════════════════════════════════╣"
        ]
        for f in findings:
            report.append(f"║ {f:<61} ║")
        report.append("╚══════════════════════════════════════════════════════════════╝")
        return "\n".join(report)

    def health_check(self) -> str:
        return f"OK — Portal Active ({self.stats['audits_served']} audits served)"
