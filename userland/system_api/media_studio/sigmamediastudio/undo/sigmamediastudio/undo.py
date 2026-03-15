# Generated method: SigmaMediaStudio.undo
import time
import os
import uuid

class SigmaMediaStudio:
    def undo(self) -> str:
        """Non-Destructive Workflow Undo."""
        if self.history_index > 0:
            self.history_index -= 1
            state = self.history[self.history_index]
            self.layers = list(state['layers'])
            self.timeline = list(state['timeline'])
            return f"Undo complete. Reverted to: {state['action']}"
        return 'Nothing to undo.'