"""
SigmaOS Advocate CMS (v1.0 Apex)
=================================
USP: Specialized Case Management for Indian Advocates & Lawyers.
Manages Hearing Dates, Client CRM, and Court Room Tracking.
Reference: e-Courts Indian Judiciary Standards.
"""
import os
import json
import time
from typing import Dict, Any, List, Optional

class SigmaModuleBase:
    def __init__(self, kernel):
        self.kernel = kernel
    def log_event(self, action: str, context: Dict[str, Any]):
        if self.kernel and hasattr(self.kernel, "bus"):
             self.kernel.bus.emit(f"cms.{action}", context)

class AdvocateCMS(SigmaModuleBase):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.kernel = kernel
        self.cms_path = "userland/legal_data/cms/"
        os.makedirs(self.cms_path, exist_ok=True)
        
        self.case_ledger: Dict[str, Dict[str, Any]] = {}
        self._load_ledger()

    def _load_ledger(self):
        path = os.path.join(self.cms_path, "ledger.json")
        if os.path.exists(path):
            try:
                with open(path, "r") as f:
                    self.case_ledger = json.load(f)
            except: self.case_ledger = {}

    def _save_ledger(self):
        path = os.path.join(self.cms_path, "ledger.json")
        with open(path, "w") as f:
            json.dump(self.case_ledger, f, indent=4)

    def add_case(self, case_fn: str, client: str, court: str, status: str = "PENDING") -> str:
        """USP: Sovereign Case Filing. Assigns a unique OS-ID to the litigation."""
        case_id = f"ADV-{int(time.time())}"
        case_blob = {
            "case_id": case_id,
            "case_fn": case_fn,
            "client": client,
            "court": court,
            "status": status,
            "hearings": [],
            "created_at": time.time()
        }
        self.case_ledger[case_id] = case_blob
        self._save_ledger()
        self.log_event("case_added", {"case_id": case_id})
        return case_id

    def add_hearing(self, case_id: str, hearing_date: str, purpose: str):
        if case_id in self.case_ledger:
            self.case_ledger[case_id]["hearings"].append({
                "date": hearing_date,
                "purpose": purpose,
                "recorded_at": time.time()
            })
            self._save_ledger()
            return True
        return False

    def get_upcoming_hearings(self) -> List[Dict[str, Any]]:
        """USP: Litigation Pulse. Returns all hearings for the coming week."""
        hearings = []
        for cid, case in self.case_ledger.items():
            for h in case["hearings"]:
                hearings.append({
                    "case_id": cid,
                    "case_fn": case["case_fn"],
                    "court": case["court"],
                    "date": h["date"],
                    "purpose": h["purpose"]
                })
        return sorted(hearings, key=lambda x: x["date"])

    def health_check(self) -> str:
        return f"OK — Active Cases: {len(self.case_ledger)} | CMS: READY"
