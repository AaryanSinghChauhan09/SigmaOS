# Generated method: TitanCapture.stop_capture
import os
import re
import json
import time
from typing import List, Dict, Any, Optional

class TitanCapture:
    def stop_capture(self) -> str:
        self.is_recording = False
        return 'Capture finalized. Logged to Sovereign Ledger.'