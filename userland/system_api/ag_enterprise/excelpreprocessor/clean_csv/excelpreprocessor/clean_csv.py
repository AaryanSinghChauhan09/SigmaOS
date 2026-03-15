# Generated method: ExcelPreprocessor.clean_csv
import os
import re
import json
import time
from typing import List, Dict, Any, Optional

class ExcelPreprocessor:
    def clean_csv(self, csv_data: str) -> str:
        cleaned = re.sub('[^\\x00-\\x7F]+', ' ', csv_data)
        return cleaned