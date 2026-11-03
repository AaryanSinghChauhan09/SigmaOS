#!/usr/bin/env python3
# SigmaOS Sticky Notes — persistent quick notes system.
import argparse, json, sys
from dataclasses import dataclass, field, asdict
from datetime import datetime
from typing import Dict, List

@dataclass
class Note:
    id: int; text: str; color: str = "yellow"; created: str = field(default_factory=lambda: datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%S"))

class NoteStore:
    def __init__(self): self._notes: Dict[int, Note] = {}; self._next_id = 1
    def add(self, text: str, color: str="yellow") -> Note:
        n = Note(self._next_id, text, color); self._notes[self._next_id] = n; self._next_id += 1; return n
    def delete(self, nid: int) -> bool: return bool(self._notes.pop(nid, None))
    def list(self) -> List[Note]: return sorted(self._notes.values(), key=lambda n: n.id)
    def get(self, nid: int) -> Note | None: return self._notes.get(nid)

STORE = NoteStore(); STORE.add("Buy groceries"); STORE.add("Review platform contracts","blue"); STORE.add("QEMU smoke test","pink")

def build_parser():
    p = argparse.ArgumentParser(prog="sigma-notes")
    sub = p.add_subparsers(dest="command", required=True)
    sub.add_parser("list",help="List all notes").set_defaults(func=lambda _: [print(f"  [{n.id}] ({n.color}) {n.text}") for n in STORE.list()] or 0)
    ad = sub.add_parser("add",help="Add a note"); ad.add_argument("text"); ad.add_argument("--color",default="yellow")
    ad.set_defaults(func=lambda a: print(f"Added note #{STORE.add(a.text,a.color).id}") or 0)
    rm = sub.add_parser("delete",help="Delete a note"); rm.add_argument("id",type=int)
    rm.set_defaults(func=lambda a: print("Deleted." if STORE.delete(a.id) else "Not found.") or 0)
    return p

if __name__ == "__main__": sys.exit(build_parser().parse_args().func(build_parser().parse_args()))
