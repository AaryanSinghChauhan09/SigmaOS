#!/usr/bin/env python3
# SigmaOS Webapp Manager — web application wrapper with browser profile management.
import argparse, sys
from dataclasses import dataclass
from typing import Dict

@dataclass
class WebApp:
    name: str
    url: str
    icon: str
    browser: str = "chromium"
    profile: str = "sigma-webapps"
    installed: bool = False

WEBAPPS: Dict[str, WebApp] = {
    "gmail":  WebApp("Gmail",  "https://mail.google.com", "gmail"),
    "github": WebApp("GitHub", "https://github.com",      "github", profile="dev-profile"),
    "notion": WebApp("Notion", "https://notion.so",       "notion"),
    "figma":  WebApp("Figma",  "https://figma.com",       "figma"),
}

def do_list() -> int:
    for name, a in WEBAPPS.items():
        state = "[installed]" if a.installed else ""
        print("  {:<15} {:<40} browser={} {}".format(name, a.url, a.browser, state))
    return 0

def do_install(name: str) -> int:
    a = WEBAPPS.get(name)
    if not a:
        print("error: '{}' not found".format(name), file=sys.stderr)
        return 1
    a.installed = True
    print("[webapp] Installed {} ({})".format(a.name, a.url))
    print("  Launch: {} --app={} --profile-directory={}".format(a.browser, a.url, a.profile))
    return 0

def do_launch(name: str) -> int:
    a = WEBAPPS.get(name)
    if not a:
        print("error: '{}' not found".format(name), file=sys.stderr)
        return 1
    print("[webapp] Launching {}...".format(a.name))
    print("  (Real impl: {} --app={} --profile-directory={})".format(a.browser, a.url, a.profile))
    return 0

def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigma-webapp")
    sub = p.add_subparsers(dest="command", required=True)
    sub.add_parser("list").set_defaults(func=lambda _: do_list())
    ins = sub.add_parser("install"); ins.add_argument("name")
    ins.set_defaults(func=lambda a: do_install(a.name))
    la = sub.add_parser("launch"); la.add_argument("name")
    la.set_defaults(func=lambda a: do_launch(a.name))
    return p

def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)

if __name__ == "__main__":
    sys.exit(main())
