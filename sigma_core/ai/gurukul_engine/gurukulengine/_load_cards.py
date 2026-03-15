# Generated method: GurukulEngine._load_cards
import time
import json
import os
from sigma_core.system.interfaces import SigmaModuleBase

class GurukulEngine:
    def _load_cards(self):
        guardian = self.kernel.registry.get('guardian')
        if guardian and guardian.is_child_mode():
            return {'A': {'q': 'What is the 1st letter of alphabet?', 'a': 'A for Apple 🍎', 'level': 0, 'next_review': 0}, 'Color_Red': {'q': 'What color is an apple?', 'a': 'RED ❤️', 'level': 0, 'next_review': 0}, 'Animal_Dog': {'q': "Which animal says 'Woof'?", 'a': 'DOG 🐶', 'level': 0, 'next_review': 0}, 'Number_1': {'q': 'How many suns are in the sky?', 'a': 'ONE (1) ☀️', 'level': 0, 'next_review': 0}}
        if os.path.exists(self.cards_path):
            with open(self.cards_path, 'r') as f:
                return json.load(f)
        return {'BNS_Section_1': {'q': 'What is BNS?', 'a': 'Bharatiya Nyaya Sanhita (New Criminal Code)', 'level': 0, 'next_review': 0}, 'DPDPA_2023': {'q': 'What is DPDPA?', 'a': 'Digital Personal Data Protection Act', 'level': 0, 'next_review': 0}}