#!/usr/bin/env python3
# SigmaOS Slide Presenter — simple slide presentation tool.
import argparse, sys
from dataclasses import dataclass, field
from typing import List

@dataclass
class Slide:
    title: str; content: str; notes: str = ""

@dataclass
class Presentation:
    title: str; slides: List[Slide] = field(default_factory=list)
    def add(self, s: Slide): self.slides.append(s)
    def current(self, idx: int): return self.slides[idx] if 0 <= idx < len(self.slides) else None

DEMO = Presentation("SigmaOS Demo", [Slide("Introduction","SigmaOS is a multi-kernel distro platform.","Key points: Linux+BSD backends"), Slide("Architecture","8-layer model: Kernel→Boot→System→Package→Desktop→Config→Security→Apps","Each layer is backend-agnostic"), Slide("Thank You","Questions?","")])

def do_info(p: Presentation): print(f"Presentation: {p.title}  ({len(p.slides)} slides)")
def do_show(p: Presentation, idx: int):
    s = p.current(idx)
    if not s: print(f"error: slide {idx} not found",file=sys.stderr); return 1
    print(f"[{idx+1}/{len(p.slides)}] {s.title}"); print(f"  {s.content}")
    if s.notes: print(f"  Notes: {s.notes}")
    return 0

def build_parser():
    p = argparse.ArgumentParser(prog="sigma-present")
    sub = p.add_subparsers(dest="command", required=True)
    sub.add_parser("info", help="Show presentation info").set_defaults(func=lambda _: do_info(DEMO) or 0)
    sc = sub.add_parser("show", help="Show a slide")
    sc.add_argument("slide", type=int, nargs="?", default=0)
    sc.set_defaults(func=lambda a: do_show(DEMO, a.slide))
    return p

if __name__ == "__main__": sys.exit(build_parser().parse_args().func(build_parser().parse_args()))
