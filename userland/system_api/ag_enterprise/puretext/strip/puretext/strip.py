# Generated method: PureText.strip
import os
import re
import json
import time
from typing import List, Dict, Any, Optional

class PureText:
    def strip(self, rich_text: str) -> str:
        return re.sub('<[^>]+>', '', rich_text)