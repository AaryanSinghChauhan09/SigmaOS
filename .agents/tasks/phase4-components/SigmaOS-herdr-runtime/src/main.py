#!/usr/bin/env python3
# SigmaOS Herdr Runtime — container/pod lifecycle orchestrator CLI scaffold
# Upstream: podman (https://podman.io/), nerdctl (https://github.com/containerd/nerdctl)
"""
SigmaOS herdr-runtime: manage container and pod lifecycles via an OCI-inspired model.
Subcommands:
  run      <image> [--name <n>] [--env K=V]...   Create and start a container
  stop     <name|id>             Stop a running container
  remove   <name|id>             Remove a stopped container
  list                           List all containers
  logs     <name|id>             Show container logs (in-memory)
  exec     <name|id> <cmd>       Run a command in a running container
  pull     <image>               Simulate pulling an image
  images                         List pulled images
  status   [<name|id>]           Show container or runtime status
"""
import argparse
import sys
import uuid
from typing import Dict, List, Optional

STATES = ("created", "running", "stopped", "exited")


class ContainerRecord:
    def __init__(self, image: str, name: Optional[str] = None, env: List[str] = None):
        self.id = str(uuid.uuid4())[:12]
        self.image = image
        self.name = name or f"herdr-{self.id[:6]}"
        self.env: List[str] = env or []
        self.state = "created"
        self.logs: List[str] = []

    def __str__(self) -> str:
        return f"{self.id[:12]}  {self.name:<20} {self.image:<30} [{self.state}]"


class HerdrRuntime:
    def __init__(self):
        self.containers: Dict[str, ContainerRecord] = {}  # id -> record
        self.name_index: Dict[str, str] = {}             # name -> id
        self.images: List[str] = []

    def _resolve(self, ref: str) -> Optional[ContainerRecord]:
        if ref in self.containers:
            return self.containers[ref]
        cid = self.name_index.get(ref)
        return self.containers.get(cid) if cid else None

    def run(self, image: str, name: Optional[str], env: List[str]) -> None:
        c = ContainerRecord(image, name, env)
        self.containers[c.id] = c
        self.name_index[c.name] = c.id
        c.state = "running"
        c.logs.append(f"Container started: image={image}")
        print(f"[herdr] Started container: {c}")

    def stop(self, ref: str) -> None:
        c = self._resolve(ref)
        if not c:
            print(f"[herdr] Unknown container: {ref}", file=sys.stderr); return
        if c.state != "running":
            print(f"[herdr] Not running: {c.name}", file=sys.stderr); return
        c.state = "stopped"
        c.logs.append("Container stopped")
        print(f"[herdr] Stopped: {c.name} ({c.id[:12]})")

    def remove(self, ref: str) -> None:
        c = self._resolve(ref)
        if not c:
            print(f"[herdr] Unknown container: {ref}", file=sys.stderr); return
        if c.state == "running":
            print(f"[herdr] Stop container first: {c.name}", file=sys.stderr); return
        del self.containers[c.id]
        self.name_index.pop(c.name, None)
        print(f"[herdr] Removed: {c.name}")

    def list_containers(self) -> None:
        if not self.containers:
            print("[herdr] No containers"); return
        print(f"{'ID':<14} {'NAME':<20} {'IMAGE':<30} STATE")
        for c in self.containers.values():
            print(f"  {c}")

    def logs(self, ref: str) -> None:
        c = self._resolve(ref)
        if not c:
            print(f"[herdr] Unknown container: {ref}", file=sys.stderr); return
        print(f"[herdr] Logs for {c.name}:")
        for line in c.logs:
            print(f"  {line}")

    def execute(self, ref: str, cmd: str) -> None:
        c = self._resolve(ref)
        if not c:
            print(f"[herdr] Unknown container: {ref}", file=sys.stderr); return
        if c.state != "running":
            print(f"[herdr] Container not running: {c.name}", file=sys.stderr); return
        entry = f"exec: {cmd}"
        c.logs.append(entry)
        print(f"[herdr] exec in {c.name}: {cmd}")
        print(f"[herdr] (scaffold: command recorded, no real execution)")

    def pull(self, image: str) -> None:
        if image not in self.images:
            self.images.append(image)
        print(f"[herdr] Pulled: {image}")

    def list_images(self) -> None:
        if not self.images:
            print("[herdr] No images"); return
        for img in self.images:
            print(f"  {img}")

    def status(self, ref: Optional[str] = None) -> None:
        if ref:
            c = self._resolve(ref)
            if not c:
                print(f"[herdr] Unknown: {ref}", file=sys.stderr); return
            print(f"ID     : {c.id}"); print(f"Name   : {c.name}")
            print(f"Image  : {c.image}"); print(f"State  : {c.state}")
            print(f"Env    : {c.env}"); print(f"Logs   : {len(c.logs)} entries")
        else:
            running = sum(1 for c in self.containers.values() if c.state == "running")
            print(f"Containers: {len(self.containers)} total, {running} running")
            print(f"Images    : {len(self.images)}")


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="sigmaos-herdr", description="SigmaOS container runtime")
    sub = p.add_subparsers(dest="cmd", required=True)

    r = sub.add_parser("run", help="Create and start a container")
    r.add_argument("image"); r.add_argument("--name", default=None)
    r.add_argument("--env", action="append", default=[], metavar="K=V")

    for nm, hlp in [("stop", "Stop a container"), ("remove", "Remove a container"),
                    ("logs", "Show container logs")]:
        sp = sub.add_parser(nm, help=hlp); sp.add_argument("ref", metavar="name|id")

    ex = sub.add_parser("exec", help="Run a command in a container")
    ex.add_argument("ref", metavar="name|id"); ex.add_argument("cmd")

    sub.add_parser("list",   help="List containers")
    sub.add_parser("images", help="List pulled images")

    pull_p = sub.add_parser("pull", help="Pull an image"); pull_p.add_argument("image")

    st = sub.add_parser("status", help="Show status"); st.add_argument("ref", nargs="?", default=None)
    return p


def main(argv: List[str] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    rt = HerdrRuntime()
    cmd = args.cmd

    if cmd == "run":       rt.run(args.image, args.name, args.env)
    elif cmd == "stop":    rt.stop(args.ref)
    elif cmd == "remove":  rt.remove(args.ref)
    elif cmd == "list":    rt.list_containers()
    elif cmd == "logs":    rt.logs(args.ref)
    elif cmd == "exec":    rt.execute(args.ref, args.cmd)
    elif cmd == "pull":    rt.pull(args.image)
    elif cmd == "images":  rt.list_images()
    elif cmd == "status":  rt.status(getattr(args, "ref", None))
    return 0


if __name__ == "__main__":
    sys.exit(main())
