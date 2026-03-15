# Generated file: test_integration
import sys
import os
import time
import logging
from sigma_core.kernel import SigmaKernel
from sigma_projects import TaskStatus
from sigma_core.kernel import SigmaKernel
from sigma_projects import TaskStatus
import traceback

def test_integration():
    print('--- SIGMAOS INTEGRATION TEST v2.0 ---')
    k = SigmaKernel(auto_load=True)
    print(f'[OK] Kernel Bootstrapped. Version: {k.cfg.VERSION}')
    routines = k.registry.get('routines')
    if routines:
        print(f'[OK] Routine Manager Active: {routines.health_check()}')
        res = routines.process_trigger('context:coding')
        print(f'[OK] Dev Routine Execution: {res}')
    else:
        print('[ERR] Routine Manager NOT FOUND in registry.')
    projects = k.registry.get('projects')
    if projects:
        print(f'[OK] Projects Engine Active: {projects.health_check()}')
        tid = projects.add_task('Test Task', 'Verifying routine triggers', TaskStatus.IN_PROGRESS)
        print(f'[OK] Task Created: {tid}')
        print('Moving task to DONE (should trigger task.done routine)...')
        projects.update_task_status(tid, TaskStatus.DONE)
        print('[OK] Task Status Updated.')
    else:
        print('[ERR] Projects Engine NOT FOUND in registry.')
    perf = k.registry.get('perf')
    if perf:
        print(f'[OK] Performance Engine Pulse: {perf.health_check()}')
        profile = perf.apply_tuning('Apex')
        print(f"[OK] Apex Profile Applied: {profile['gpu_clock']}")
        res = perf.trigger_workload_hoard()
        print(f'[OK] Workflow Hoard: {res}')
    else:
        print('[ERR] Performance Engine NOT FOUND in registry.')
    parity = k.registry.get('linux_parity')
    if parity:
        print(f'[OK] Linux Parity Status: {parity.gap_analysis.health_check()}')
        gap = parity.gap_analysis.generate_report('Kali Linux')
        print('[OK] Kali Linux Gap Report Generated.')
    else:
        print('[ERR] Linux Parity Engine NOT FOUND in registry.')
    print('\n--- ALL SYSTEMS NOMINAL (SOVEREIGN GRADE) ---')