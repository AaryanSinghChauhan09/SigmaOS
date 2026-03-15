# Generated method: SigmaExplorer.tag_file
import os
import time

class SigmaExplorer:
    def tag_file(self, file_path: str, tag: str) -> str:
        """USP: Graph-based tagging similar to Spacedrive."""
        if tag not in self.virtual_tags:
            self.virtual_tags[tag] = []
        self.virtual_tags[tag].append(file_path)
        return f"File '{file_path}' tagged with '{tag}'."