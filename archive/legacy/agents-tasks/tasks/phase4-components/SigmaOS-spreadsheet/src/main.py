#!/usr/bin/env python3
# SigmaOS Spreadsheet — cell-based data entry with basic formulas.
import argparse, sys
from typing import Dict, Optional, Union

Cell = Optional[Union[float, str]]

class Spreadsheet:
    def __init__(self): self._data: Dict[str, Cell] = {}
    def _key(self, row: int, col: int) -> str:
        return chr(65 + col) + str(row + 1)
    def set(self, row: int, col: int, v: Cell):
        self._data[self._key(row, col)] = v
    def get(self, row: int, col: int) -> Cell:
        return self._data.get(self._key(row, col))
    def sum_col(self, col: int, start: int, end: int) -> float:
        return sum(float(self.get(r, col) or 0) for r in range(start, end))
    def to_csv(self, rows: int = 5, cols: int = 4) -> str:
        lines = []
        for r in range(rows):
            cells = [str(self.get(r, c) if self.get(r, c) is not None else "") for c in range(cols)]
            lines.append(",".join(cells))
        return "\n".join(lines)

SHEET = Spreadsheet()
SHEET.set(0, 0, "Name"); SHEET.set(0, 1, "Q1"); SHEET.set(0, 2, "Q2"); SHEET.set(0, 3, "Total")
SHEET.set(1, 0, "Alpha"); SHEET.set(1, 1, 1200.0); SHEET.set(1, 2, 1500.0); SHEET.set(1, 3, 2700.0)
SHEET.set(2, 0, "Beta");  SHEET.set(2, 1,  800.0); SHEET.set(2, 2,  950.0); SHEET.set(2, 3, 1750.0)

def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigma-sheet", description="SigmaOS spreadsheet CLI")
    sub = p.add_subparsers(dest="command", required=True)

    sub.add_parser("show", help="Print sheet as CSV").set_defaults(func=lambda _: print(SHEET.to_csv()) or 0)

    sc = sub.add_parser("set", help="Set a cell value")
    sc.add_argument("row", type=int); sc.add_argument("col", type=int); sc.add_argument("value")
    def do_set(a):
        SHEET.set(a.row, a.col, a.value)
        print("Set " + chr(65 + a.col) + str(a.row + 1) + "=" + a.value)
        return 0
    sc.set_defaults(func=do_set)

    sm = sub.add_parser("sum", help="Sum a column range")
    sm.add_argument("col", type=int); sm.add_argument("start", type=int, default=1, nargs="?"); sm.add_argument("end", type=int, default=3, nargs="?")
    def do_sum(a):
        total = SHEET.sum_col(a.col, a.start, a.end)
        print("Sum col " + chr(65 + a.col) + ": " + str(total))
        return 0
    sm.set_defaults(func=do_sum)

    return p

def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)

if __name__ == "__main__":
    sys.exit(main())
