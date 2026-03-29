"""
SigmaOS Apex Shard: launch_distro
"""
import os
import subprocess
import sys
from .launch_distro_shards.check_qemu import check_qemu

def launch_distro(distro_id):
    if distro_id not in DISTROS:
        print(f"ERROR: Shard '{distro_id}' not found in Industrial Repository.")
        return
    distro = DISTROS[distro_id]
    print(f"[LAUNCH] Initiating {distro['name']} silicon shard...")
    if not check_qemu():
        print(f'[WARNING] Local QEMU not found. Falling back to WASM Aether Shard...')
        print(f'[SHARD] URL: https://copy.sh/v86/?profile={distro_id}')
        return
    print(f"[SHARD] RAM: {distro['ram']}MB | ISO Source: {distro['iso']}")
    print(f"[SHARD] Instruction: qemu-system-x86_64 -m {distro['ram']} -cdrom {distro_id}.iso -boot d")
    print('[OK] Distribution Shard: READY FOR SILICON.')