# Generated method: SigmaSupportEcosystem._format_table_for_whatsapp
import time
import secrets
import random
from dataclasses import dataclass
from enum import Enum, auto

class SigmaSupportEcosystem:
    def _format_table_for_whatsapp(self, table_str: str) -> str:
        """Converts Markdown tables to WhatsApp-friendly bolded lists."""
        lines = table_str.split('\n')
        formatted = []
        for line in lines:
            if '|' in line and '---' not in line:
                cells = [c.strip() for c in line.split('|') if c.strip()]
                if len(cells) >= 2:
                    formatted.append(f'• *{cells[0]}*: {cells[1]}')
            else:
                formatted.append(line)
        return '\n'.join(formatted)