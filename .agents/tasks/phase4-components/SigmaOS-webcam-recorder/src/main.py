#!/usr/bin/env python3
# SigmaOS Webcam Recorder — webcam capture and recording utility.
import argparse, sys
from dataclasses import dataclass
from typing import List, Optional

@dataclass
class WebcamDevice:
    index: int; name: str; resolutions: List[str]; fps: int = 30

DEVICES: List[WebcamDevice] = [
    WebcamDevice(0,"Integrated Webcam",["1280x720","1920x1080","640x480"]),
    WebcamDevice(1,"Logitech C920",["1920x1080","2560x1440","640x480"],fps=60),
]

recording: Optional[str] = None

def do_list(): [print(f"  /dev/video{d.index}  {d.name}  res={','.join(d.resolutions[:2])}  fps={d.fps}") for d in DEVICES]
def do_record(dev_idx: int, res: str, out: str):
    if dev_idx >= len(DEVICES): print("error: invalid device",file=sys.stderr); return 1
    d=DEVICES[dev_idx]; print(f"[webcam] Recording {d.name} at {res}"); print(f"  (Real impl: ffmpeg -f v4l2 -r {d.fps} -s {res} -i /dev/video{d.index} {out})")
    return 0

def build_parser():
    p = argparse.ArgumentParser(prog="sigma-webcam")
    sub = p.add_subparsers(dest="command", required=True)
    sub.add_parser("list").set_defaults(func=lambda _: do_list() or 0)
    rec = sub.add_parser("record"); rec.add_argument("--device",type=int,default=0); rec.add_argument("--res",default="1280x720"); rec.add_argument("--out",default="/tmp/webcam.mp4")
    rec.set_defaults(func=lambda a: do_record(a.device,a.res,a.out))
    return p

if __name__ == "__main__": sys.exit(build_parser().parse_args().func(build_parser().parse_args()))
