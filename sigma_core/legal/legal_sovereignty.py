"""
SigmaOS Legal Sovereignty Shard (v1.0 Apex)
===========================================
USP: Procedural & Substantive Compliance for Indian Laws (BNS, BNSS, BSA).
Orchestrates statutory timelines and evidentiary admissibility audits.
"""
import time
from typing import Dict, Any, List, Optional

try:
    from sigma_core.system.interfaces import SigmaModuleBase
except (ImportError, ValueError):
    class SigmaModuleBase:
        def __init__(self, kernel): self.kernel = kernel
        def log_event(self, a, c): pass

class LegalSovereignty(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.kernel = kernel
        self.codes = {
            "BNSS": "Bharatiya Nagarik Suraksha Sanhita (2023) - Procedural",
            "BNS":  "Bharatiya Nyaya Sanhita (2023) - Substantive",
            "BSA":  "Bharatiya Sakshya Adhiniyam (2023) - Evidence"
        }
        
        # Statutory timelines under BNSS (Simplified)
        self.deadlines = {
            "charge_sheet_minor": 60.0, # Days
            "charge_sheet_major": 90.0, # Days
            "judgment_after_args": 30.0, # Days
            "appeal_limitation": 30.0   # Days (Standard)
        }

    def audit_compliance(self, case_stage: str, start_time: float) -> Dict[str, Any]:
        """USP: Proactive statutory auditing against BNSS/BNS/BSA."""
        elapsed_days = (time.time() - start_time) / (24 * 3600)
        limit = float(self.deadlines.get(case_stage, 999.0))
        
        status = "COMPLIANT"
        if elapsed_days > limit:
            status = "DELAYED"
        elif elapsed_days > limit * 0.8:
            status = "WARNING"
            
        return {
            "status": status,
            "elapsed": round(float(elapsed_days), 1),
            "limit": limit,
            "deviation": max(0.0, round(float(elapsed_days - limit), 1))
        }

    def get_stage_definition(self, code: str, stage: str) -> str:
        """USP: Intelligent Legal Dictionary (Supreme Court Aligned)."""
        definitions = {
            "BNSS": {
                "FIR": "Section 173: Information in cognizable cases.",
                "INVESTIGATION": "Section 175: Police officer's power to investigate.",
                "CHARGES": "Section 243: Framing of charge."
            }
        }
        return definitions.get(code, {}).get(stage, "Definition not found in local shard cache.")

    def health_check(self) -> str:
        return "OK — Legal Sovereignty Shard (BNS/BNSS/BSA) Hydrated."
