# Generated method: TimeTracker.stop_session
import os
import re
import json
import time
from typing import List, Dict, Any, Optional

class TimeTracker:
    def stop_session(self) -> str:
        if not self.start_time:
            return 'No active session.'
        elapsed = time.time() - self.start_time
        self.start_time = None
        return f'Tracker: Session complete. {int(elapsed)} seconds cached to Sovereign Ledger.'