# Generated method: SigmaWriteSense.analyze_readability
from typing import Dict, List, Any
import re

class SigmaWriteSense:
    def analyze_readability(self, text: str) -> Dict:
        """USP: Hemingway Editor style analysis (Readability & Clarity)."""
        sentences = re.split('[.!?]+', text)
        words = text.split()
        complex_count = sum((1 for s in sentences if len(s.split()) > 20))
        passive_voice = len(re.findall('\\b(am|is|are|was|were|be|been|being)\\b\\s+(\\w+ed|seen|known|found)', text.lower()))
        adverb_count = len(re.findall('\\b\\w+ly\\b', text.lower()))
        grade = max(5, int(len(words) / 10))
        return {'Grade_Level': grade, 'Verdict': 'Good' if grade < 10 else 'Hard to Read', 'Complex_Sentences': complex_count, 'Passive_Voice_Instances': passive_voice, 'Adverbs_Detected': adverb_count, 'Recommendation': 'Simplify bolded sentences.' if complex_count > 0 else 'Flow is optimal.'}