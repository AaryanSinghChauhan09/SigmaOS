# Generated file: main
import sys
import os
import argparse
from shared_processor import SigmaSharedProcessor

def main():
    parser = argparse.ArgumentParser(description='SigmaOS AetherGrid: Distributed Processing CLI')
    subparsers = parser.add_subparsers(dest='command')
    subparsers.add_parser('discover', help='Discover local SigmaOS peers on the network mesh.')
    dist_parser = subparsers.add_parser('distribute', help='Offload a task to the AetherGrid.')
    dist_parser.add_argument('task', help='Name of the task to distribute')
    dist_parser.add_argument('complexity', type=int, help='Complexity score (1-100)')
    subparsers.add_parser('audit', help='View the immutable execution ledger.')
    args = parser.parse_args()
    grid = SigmaSharedProcessor()
    if args.command == 'discover':
        print(grid.discover_local_peers())
    elif args.command == 'distribute':
        print(grid.distribute_workload(args.task, args.complexity))
    elif args.command == 'audit':
        ledger = grid.get_compliance_audit_trail()
        if not ledger:
            print('Audit: No tasks recorded in the sovereign ledger.')
        else:
            print('--- IMMUTABLE EXECUTION LEDGER ---')
            for entry in ledger:
                print(f"[{entry['timestamp']}] Task: {entry['task']} | Dest: {entry['destination']} | Sig: {entry['signature'][:16]}...")
    else:
        parser.print_help()