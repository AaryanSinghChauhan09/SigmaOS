#!/usr/bin/env python3
import sys
import json
import argparse

def main():
    parser = argparse.ArgumentParser(description="Generate benchmark report")
    parser.add_argument("--boot", help="Boot benchmark JSON file")
    parser.add_argument("--memory", help="Memory benchmark JSON file")
    parser.add_argument("--output", help="Output report file")
    args = parser.parse_args()

    report = {"status": "success", "benchmarks": {}}
    if args.boot:
        try:
            with open(args.boot, "r") as f:
                report["benchmarks"]["boot"] = json.load(f)
        except Exception as e:
            report["benchmarks"]["boot_error"] = str(e)

    if args.memory:
        try:
            with open(args.memory, "r") as f:
                report["benchmarks"]["memory"] = json.load(f)
        except Exception as e:
            report["benchmarks"]["memory_error"] = str(e)

    output_path = args.output if args.output else "benchmark-report.json"
    with open(output_path, "w") as f:
        json.dump(report, f, indent=2)

    print(f"Generated benchmark report at {output_path}")

if __name__ == "__main__":
    main()
