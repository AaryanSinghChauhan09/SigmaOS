# Generated method: SigmaManual.search
from typing import Dict, List, Any

class SigmaManual:
    def search(self, query: str) -> List[Dict[str, str]]:
        results = []
        query = query.lower()
        for section, topics in self.MANUAL_DATA.items():
            for topic, text in topics.items():
                if query in section.lower() or query in topic.lower() or query in text.lower():
                    results.append({'section': section, 'topic': topic, 'content': text})
        return results