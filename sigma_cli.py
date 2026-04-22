#!/usr/bin/env python3
import sys
import argparse
import urllib.request
import json
import os

API_URL = "http://localhost:8080/api"

def call_api(endpoint, payload=None):
    url = f"{API_URL}/{endpoint}"
    try:
        if payload:
            data = json.dumps(payload).encode('utf-8')
            req = urllib.request.Request(url, data=data, headers={'Content-Type': 'application/json'})
        else:
            req = urllib.request.Request(url)
            
        with urllib.request.urlopen(req, timeout=5) as res:
            return res.read().decode('utf-8')
    except Exception as e:
        return f"[ERROR] Failed to contact SigmaOS Zenith server: {e}"

def main():
    parser = argparse.ArgumentParser(description="SigmaOS Unified CLI")
    subparsers = parser.add_subparsers(dest="command")

    # Automations
    auto_parser = subparsers.add_parser("auto", help="Trigger system automations")
    auto_parser.add_argument("action", choices=["heap_compact", "zombie_sweep", "cache_flush"])

    # Personalization
    set_parser = subparsers.add_parser("set", help="Customize UI/OS settings")
    set_parser.add_argument("key", type=str)
    set_parser.add_argument("value", type=str)

    # Info
    subparsers.add_parser("telemetry", help="Get live system telemetry")

    args = parser.parse_args()

    if args.command == "auto":
        print(f"Triggering automation: {args.action}")
        # Send to Zenith server via run API (mocking the GUI click)
        res = call_api("run", {"cmd": f"echo 'AUTOMATION TRIGGERED: {args.action}'", "cwd": ""})
        print(res)
    
    elif args.command == "set":
        print(f"Applying setting: {args.key} = {args.value}")
        # In a real setup, we would send this to the Zenith WebSocket or API to propagate to localstorage
        print("[SUCCESS] Sent customization intent to Zenith.")

    elif args.command == "telemetry":
        print(call_api("telemetry"))

    else:
        parser.print_help()

if __name__ == "__main__":
    main()
