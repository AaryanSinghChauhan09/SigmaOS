# Generated file: audit
import sys
import os
import inspect
from sigma_core import SigmaKernel

def audit():
    print('=' * 60)
    print(' SIGMAOS KERNEL MODULE DEEP AUDIT')
    print('=' * 60)
    k = SigmaKernel(auto_load=True)
    modules_to_check = {'watchdog': 'watchdog', 'shadow': 'shadow', 'crusher': 'crusher', 'pbs': 'pbs', 'intelligence': 'intel', 'kad': 'kad', 'crash_reporter': 'crash_reporter', 'energy': 'energy', 'update_manager': 'update_manager', 'perf': 'perf', 'memory': 'memory', 'process': 'process', 'fs': 'fs', 'net_guard': 'net_guard', 'repair_engine': 'repair_engine', 'healer': 'healer', 'aura': 'aura', 'prewarmer': 'prewarmer', 'sandbox': 'sandbox', 'polyglot_runtime': 'polyglot_runtime'}
    results = []
    for reg_name, prop_name in modules_to_check.items():
        obj = getattr(k, prop_name, None)
        if obj is None:
            obj = k.registry.get(reg_name)
        if obj is None:
            results.append(f'[FAIL] {reg_name:18}: NOT LOADED')
        else:
            try:
                try:
                    source_file = inspect.getfile(obj.__class__)
                except:
                    source_file = 'Unknown (Built-in or dynamic)'
                status = 'LOADED'
                if hasattr(obj, 'health_check'):
                    try:
                        status = obj.health_check()
                    except Exception as e:
                        status = f'HEALTH_CHECK_ERROR: {str(e)}'
                else:
                    status = 'MISSING_HEALTH_CHECK'
                results.append(f'[OK]   {reg_name:18}: {status[:50]} \n       (Source: {source_file})')
            except Exception as e:
                results.append(f'[ERROR] {reg_name:18}: {str(e)}')
    for r in results:
        print(r)
    print('=' * 60)
    print(f'Total Modules Scanned: {len(modules_to_check)}')
    print(f"Operational: {len([r for r in results if '[OK]' in r])}")
    print('=' * 60)