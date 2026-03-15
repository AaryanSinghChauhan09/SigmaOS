# Generated file: main
import sys
import os
from sigma_core.kernel import SigmaKernel
from sigma_core.manifest import CORE_SYSTEM_MODULES

def main():
    print('--- SIGMAOS KERNEL INTEGRITY AUDIT ---')
    kernel = SigmaKernel()
    print(f'\n[AUDIT] Validating {len(CORE_SYSTEM_MODULES)} system shards...')
    missing: list[str] = []
    healthy: int = 0
    for mod_path, class_name, shard_id in CORE_SYSTEM_MODULES:
        try:
            if hasattr(kernel, shard_id):
                shard = getattr(kernel, shard_id)
                health = 'UNKNOWN'
                if hasattr(shard, 'health_check'):
                    health = shard.health_check()
                print(f'  ✔ {shard_id:<18} | Status: {health}')
                healthy = healthy + 1
            else:
                print(f'  ✘ {shard_id:<18} | Status: MISSING_FROM_GRID')
                missing.append(shard_id)
        except Exception as e:
            print(f'  ⚠ {shard_id:<18} | Status: LOAD_ERROR ({e})')
            missing.append(shard_id)
    print(f'\n--- AUDIT RESULTS ---')
    print(f'  TOTAL: {len(CORE_SYSTEM_MODULES)}')
    print(f'  HEALTHY: {healthy}')
    print(f'  FAILURES: {len(missing)}')
    if missing:
        print(f"\n[CRITICAL] Missing Shards: {', '.join(missing)}")
        sys.exit(1)
    else:
        print('\n[SUCCESS] Kernel Tactical Grid is fully hydrated.')
        sys.exit(0)