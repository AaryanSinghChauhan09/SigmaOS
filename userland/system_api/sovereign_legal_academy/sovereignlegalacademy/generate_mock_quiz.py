# Generated method: SovereignLegalAcademy.generate_mock_quiz
import time
import json
import random
from typing import List, Dict, Any, Optional

class SovereignLegalAcademy:
    def generate_mock_quiz(self) -> List[Dict[str, str]]:
        """USP: Exam Mastery. Generates questions from the local index."""
        questions = []
        for act, data in self.legal_index.items():
            if 'key_sections' in data:
                sec, desc = random.choice(list(data['key_sections'].items()))
                questions.append({'type': 'Legal', 'question': f'What does {act} Section {sec} cover?', 'answer': desc})
        return questions