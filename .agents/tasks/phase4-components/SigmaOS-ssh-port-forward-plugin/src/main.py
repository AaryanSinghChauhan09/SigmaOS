#!/usr/bin/env python3
# SigmaOS SSH Port Forward Plugin — tunnel management CLI scaffold
# Upstream: openssh port-forwarding, sshuttle (https://github.com/sshuttle/sshuttle)
"""
SigmaOS ssh-port-forward-plugin: manage SSH port-forward tunnels.
Subcommands:
  add      <name> --host <h> --user <u> --local <lport> --remote <rport>
                           Add a tunnel definition
  remove   <name>          Remove a tunnel definition
  start    <name>          Start (activate) a tunnel
  stop     <name>          Stop a tunnel
  list                     List all tunnel definitions and their state
  status   [<name>]        Show status of one or all tunnels
  export   [--format json|text]   Export tunnel definitions
"""
import argparse
import json
import sys
from typing import Dict, List, Optional

TUNNEL_TYPES = ("local", "remote", "dynamic")


class TunnelDefinition:
    def __init__(self, name: str, host: str, user: str,
                 local_port: int, remote_port: int,
                 tunnel_type: str = "local",
                 identity: Optional[str] = None):
        self.name = name
        self.host = host
        self.user = user
        self.local_port = local_port
        self.remote_port = remote_port
        self.tunnel_type = tunnel_type
        self.identity = identity
        self.state = "stopped"  # stopped | running | error

    def ssh_cmd(self) -> str:
        flags = "-N -f"
        if self.identity:
            flags += f" -i {self.identity}"
        if self.tunnel_type == "local":
            fwd = f"-L {self.local_port}:localhost:{self.remote_port}"
        elif self.tunnel_type == "remote":
            fwd = f"-R {self.remote_port}:localhost:{self.local_port}"
        else:  # dynamic / SOCKS
            fwd = f"-D {self.local_port}"
        return f"ssh {flags} {fwd} {self.user}@{self.host}"

    def __str__(self) -> str:
        return (f"{self.name:<20} [{self.state:<7}] "
                f"{self.tunnel_type:<7} "
                f":{self.local_port} ↔ {self.user}@{self.host}:{self.remote_port}")

    def to_dict(self) -> dict:
        return {
            "name": self.name, "host": self.host, "user": self.user,
            "local_port": self.local_port, "remote_port": self.remote_port,
            "tunnel_type": self.tunnel_type, "identity": self.identity,
            "state": self.state,
        }


class TunnelManager:
    def __init__(self):
        self.tunnels: Dict[str, TunnelDefinition] = {}

    def add(self, name: str, host: str, user: str,
            local_port: int, remote_port: int,
            tunnel_type: str = "local",
            identity: Optional[str] = None) -> None:
        if name in self.tunnels:
            print(f"[ssh-fwd] Tunnel '{name}' already exists", file=sys.stderr)
            return
        t = TunnelDefinition(name, host, user, local_port, remote_port, tunnel_type, identity)
        self.tunnels[name] = t
        print(f"[ssh-fwd] Added: {t}")
        print(f"          SSH cmd: {t.ssh_cmd()}")

    def remove(self, name: str) -> None:
        if name not in self.tunnels:
            print(f"[ssh-fwd] Unknown tunnel: {name}", file=sys.stderr)
            return
        if self.tunnels[name].state == "running":
            print(f"[ssh-fwd] Stop tunnel first: {name}", file=sys.stderr)
            return
        del self.tunnels[name]
        print(f"[ssh-fwd] Removed: {name}")

    def start(self, name: str) -> None:
        if name not in self.tunnels:
            print(f"[ssh-fwd] Unknown tunnel: {name}", file=sys.stderr)
            return
        t = self.tunnels[name]
        t.state = "running"
        print(f"[ssh-fwd] Started: {t}")
        print(f"          SSH cmd: {t.ssh_cmd()}")

    def stop(self, name: str) -> None:
        if name not in self.tunnels:
            print(f"[ssh-fwd] Unknown tunnel: {name}", file=sys.stderr)
            return
        self.tunnels[name].state = "stopped"
        print(f"[ssh-fwd] Stopped: {name}")

    def list_tunnels(self) -> None:
        if not self.tunnels:
            print("[ssh-fwd] No tunnels defined")
            return
        for t in self.tunnels.values():
            print(f"  {t}")

    def status(self, name: Optional[str] = None) -> None:
        targets = ([self.tunnels[name]] if name and name in self.tunnels
                   else list(self.tunnels.values()))
        if not targets:
            print(f"[ssh-fwd] Unknown tunnel: {name}" if name else "[ssh-fwd] No tunnels")
            return
        running = sum(1 for t in self.tunnels.values() if t.state == "running")
        print(f"Tunnels: {len(self.tunnels)} defined, {running} running")
        for t in targets:
            print(f"  {t}")

    def export(self, fmt: str = "text") -> None:
        if fmt == "json":
            print(json.dumps([t.to_dict() for t in self.tunnels.values()], indent=2))
        else:
            for t in self.tunnels.values():
                print(f"  {t}")
                print(f"    ssh: {t.ssh_cmd()}")


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigmaos-ssh-port-forward",
                                description="SigmaOS SSH tunnel manager")
    sub = p.add_subparsers(dest="cmd", required=True)

    a = sub.add_parser("add", help="Add a tunnel definition")
    a.add_argument("name"); a.add_argument("--host", required=True)
    a.add_argument("--user", required=True)
    a.add_argument("--local",  dest="local_port",  type=int, required=True)
    a.add_argument("--remote", dest="remote_port", type=int, required=True)
    a.add_argument("--type",   dest="tunnel_type", choices=TUNNEL_TYPES, default="local")
    a.add_argument("--identity", default=None)

    rm = sub.add_parser("remove", help="Remove a tunnel"); rm.add_argument("name")
    st = sub.add_parser("start",  help="Start a tunnel");  st.add_argument("name")
    sp = sub.add_parser("stop",   help="Stop a tunnel");   sp.add_argument("name")
    sub.add_parser("list", help="List all tunnels")

    sx = sub.add_parser("status", help="Show status")
    sx.add_argument("name", nargs="?", default=None)

    ex = sub.add_parser("export", help="Export tunnel definitions")
    ex.add_argument("--format", choices=["json", "text"], default="text")

    return p


def main(argv: List[str] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    mgr = TunnelManager()

    if args.cmd == "add":
        mgr.add(args.name, args.host, args.user,
                args.local_port, args.remote_port,
                args.tunnel_type, args.identity)
    elif args.cmd == "remove":
        mgr.remove(args.name)
    elif args.cmd == "start":
        mgr.start(args.name)
    elif args.cmd == "stop":
        mgr.stop(args.name)
    elif args.cmd == "list":
        mgr.list_tunnels()
    elif args.cmd == "status":
        mgr.status(getattr(args, "name", None))
    elif args.cmd == "export":
        mgr.export(args.format)
    return 0


if __name__ == "__main__":
    sys.exit(main())
