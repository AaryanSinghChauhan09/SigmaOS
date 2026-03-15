# Generated method: SigmaMediaStudio._record_state
import time
import os
import uuid

class SigmaMediaStudio:
    def _record_state(self, action_name: str):
        """Records state for non-destructive undo/redo."""
        if self.history_index < len(self.history) - 1:
            self.history = self.history[:self.history_index + 1]
        state_snapshot = {'action': action_name, 'layers': list(self.layers), 'timeline': list(self.timeline)}
        self.history.append(state_snapshot)
        self.history_index += 1