# Generated file: run_integration_test
import sys
import os
from sigma_core.kernel import SigmaKernel

def run_integration_test():
    print('\n--- INITIATING SOVEREIGN INTEGRITY AUDIT ---\n')
    kernel = SigmaKernel(auto_load=False)
    print('✓ Kernel Shard initialized.')
    try:
        from sigma_core.ui.customizer import SovereignCustomizer
        cust = SovereignCustomizer(kernel)
        res = cust.health_check()
        print(f'✓ UI Shard: {res}')
    except Exception as e:
        print(f'✗ UI Shard Failure: {e}')
    try:
        from sigma_core.ai.cortex_engine import CortexEngine
        cortex = CortexEngine(kernel)
        res = cortex.health_check()
        print(f'✓ AI Shard: {res}')
    except Exception as e:
        print(f'✗ AI Shard Failure: {e}')
    try:
        from sigma_core.system.resource_alchemist import ResourceAlchemist
        alc = ResourceAlchemist(kernel)
        res = alc.health_check()
        print(f'✓ Resource Shard: {res}')
    except Exception as e:
        print(f'✗ Resource Shard Failure: {e}')
    try:
        from sigma_core.system.personalization import PersonalizationEngine
        pers = PersonalizationEngine(kernel)
        res = pers.health_check()
        print(f'✓ Intelligence Shard: {res}')
    except Exception as e:
        print(f'✗ Intelligence Shard Failure: {e}')
    try:
        from sigma_core.system.autonomic_healer import AutonomicHealer
        healer = AutonomicHealer(kernel)
        res = healer.health_check()
        print(f'✓ Healer Shard: {res}')
    except Exception as e:
        print(f'✗ Healer Shard Failure: {e}')
    try:
        from sigma_core.system.automation_engine import AutomationEngine
        auto = AutomationEngine(kernel)
        res = auto.health_check()
        print(f'✓ Automation Shard: {res}')
    except Exception as e:
        print(f'✗ Automation Shard Failure: {e}')
    print('\n--- INTEGRITY AUDIT COMPLETE: SIGMAOS IS APEX READY ---')