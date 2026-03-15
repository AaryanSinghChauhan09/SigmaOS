"""
Auto-split from userland\system_api\media_studio.py — SigmaMediaStudio.new_project
"""

import time
import os
import uuid



class SigmaMediaStudio:
    def new_project(self, project_name: str, p_type: str='Image') -> str:
        self.active_project = project_name
        self.project_type = p_type
        self.layers = []
        self.timeline = []
        self.history = []
        self.history_index = -1
        self._record_state('Project Initialized')
        return f"Initialized new {p_type} project: '{project_name}'. Operating in Zero-Trust Sandbox."
