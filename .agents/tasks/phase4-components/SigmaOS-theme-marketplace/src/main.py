#!/usr/bin/env python3
# SigmaOS Theme Marketplace — curated theme install flow with trust validation.
import argparse, sys
from dataclasses import dataclass
from typing import Dict

@dataclass
class MarketplaceTheme:
    name: str; description: str; author: str; votes: int; trust_score: float; version: str; installed: bool = False

CATALOGUE: Dict[str, MarketplaceTheme] = {
    "catppuccin-mocha": MarketplaceTheme("catppuccin-mocha","Soothing dark theme","catppuccin",1200,0.97,"0.3.1"),
    "everforest-dark":  MarketplaceTheme("everforest-dark","Nature-inspired dark theme","sainnhe",890,0.93,"1.0.1"),
    "rose-pine":        MarketplaceTheme("rose-pine","All natural pine","rose-pine",756,0.91,"1.3.0"),
    "dracula-sigma":    MarketplaceTheme("dracula-sigma","Dracula for SigmaOS","community",320,0.78,"0.5.0"),
}

def do_search(q): [print(f"  {t.name:<25} v{t.version}  votes={t.votes}  trust={t.trust_score:.0%}  {t.description}") for t in CATALOGUE.values() if q.lower() in t.name or q.lower() in t.description]
def do_install(name, allow_untrusted=False):
    t=CATALOGUE.get(name)
    if not t: print(f"error: '{name}' not found",file=sys.stderr); return 1
    if t.trust_score < 0.7 and not allow_untrusted: print(f"error: low trust score ({t.trust_score:.0%}). Use --allow-untrusted",file=sys.stderr); return 1
    t.installed = True; print(f"[marketplace] Installed {t.name} v{t.version}"); return 0

def build_parser():
    p = argparse.ArgumentParser(prog="sigma-theme-market")
    sub = p.add_subparsers(dest="command", required=True)
    se = sub.add_parser("search"); se.add_argument("query"); se.set_defaults(func=lambda a: do_search(a.query) or 0)
    ins = sub.add_parser("install"); ins.add_argument("name"); ins.add_argument("--allow-untrusted",action="store_true")
    ins.set_defaults(func=lambda a: do_install(a.name, a.allow_untrusted))
    return p

if __name__ == "__main__": sys.exit(build_parser().parse_args().func(build_parser().parse_args()))
