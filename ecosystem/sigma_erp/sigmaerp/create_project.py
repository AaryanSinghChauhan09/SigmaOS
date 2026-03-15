# Generated method: SigmaERP.create_project
from typing import Dict, List, Any
import uuid

class SigmaERP:
    def create_project(self, name: str, milestones: List[str]) -> str:
        proj_id = str(uuid.uuid4())[:8]
        self._projects[proj_id] = {'name': name, 'milestones': milestones, 'progress': 0}
        return f"Projects: '{name}' initialized. {len(milestones)} milestones indexed."