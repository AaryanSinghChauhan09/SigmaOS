"""
SigmaOS Gurukul Learning Engine (v1.0 Apex)
============================================
USP: Integrated Spaced Repetition (SRS) + Bharat Law Knowledge Mapping.
Absorbs USP of: Anki (integrated), Duolingo (gamified), and Notion (educational).
"""
import time
import json
import os
from sigma_core.system.interfaces import SigmaModuleBase

class GurukulEngine(SigmaModuleBase):
    def __init__(self, kernel):
        super().__init__(kernel)
        self.cards_path = os.path.join(os.path.dirname(__file__), "..", "..", "userland", "gurukul_cards.json")
        self.knowledge_base = self._load_cards()
        self.stats = {
            "retention_rate": 0.85,
            "concepts_mastered": 0,
            "streak_days": 1
        }

    def _load_cards(self):
        if os.path.exists(self.cards_path):
            with open(self.cards_path, "r") as f:
                return json.load(f)
        return {
            "BNS_Section_1": {"q": "What is BNS?", "a": "Bharatiya Nyaya Sanhita (New Criminal Code)", "level": 0, "next_review": 0},
            "DPDPA_2023": {"q": "What is DPDPA?", "a": "Digital Personal Data Protection Act", "level": 0, "next_review": 0}
        }

    def review_concept(self, card_id: str, success: bool):
        """USP: Integrated SRS Logic for Indian Law & CS."""
        if card_id not in self.knowledge_base: return
        
        card = self.knowledge_base[card_id]
        if success:
            card["level"] += 1
            interval = (2 ** card["level"]) * 86400 
            self.stats["concepts_mastered"] += 1
        else:
            card["level"] = 0
            interval = 3600
            
        card["next_review"] = time.time() + interval
        self._save_cards()
        return f"Concept {card_id} scheduled for review."

    def _save_cards(self):
        if not os.path.exists(os.path.dirname(self.cards_path)):
             os.makedirs(os.path.dirname(self.cards_path), exist_ok=True)
        with open(self.cards_path, "w") as f:
            json.dump(self.knowledge_base, f, indent=4)

    def get_due_concepts(self):
        now = time.time()
        return [cid for cid, c in self.knowledge_base.items() if c["next_review"] <= now]

    def health_check(self) -> str:
        due = len(self.get_due_concepts())
        return f"OK - Mastered: {self.stats['concepts_mastered']} | Due: {due}"
