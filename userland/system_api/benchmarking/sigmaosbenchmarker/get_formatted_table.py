# Generated method: SigmaOSBenchmarker.get_formatted_table
from __future__ import annotations
from typing import Dict, List, Any

class SigmaOSBenchmarker:
    @classmethod
    def get_formatted_table(cls) -> str:
        col_w = 14
        header = f"{'Dimension':<18}" + ''.join((f'{k:>{col_w}}' for k in cls.SCORES))
        sep = '-' * len(header)
        rows = [header, sep]
        for i, dim in enumerate(cls.DIMENSIONS):
            row = f'{dim:<18}' + ''.join((f'{cls.SCORES[k][i]:>{col_w}}' for k in cls.SCORES))
            rows.append(row)
        return '\n'.join(rows)