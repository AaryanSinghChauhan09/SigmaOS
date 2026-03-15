# Generated method: MockKernel.__init__
import time
from typing import Dict, List, Any, Optional

class MockKernel:
    def __init__(self):
        from .automation_brain import AutomationBrain
        self.registry = {'automation_brain': AutomationBrain(self)}