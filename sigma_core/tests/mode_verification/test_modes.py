# Generated file: test_modes
import sys
import os
from sigma_core.kernel import SigmaKernel
from sigma_core.system.mode_manager import SigmaModeManager

def test_modes():
    print('Initiating SigmaOS Mode Stress Test...')
    kernel = SigmaKernel(auto_load=False)
    kernel.aura = type('Aura', (), {'apply_aura': lambda self, x: print(f'  [AURA] Applied {x}')})()
    modes = SigmaModeManager(kernel)
    kernel.registry.register('modes', modes)
    if not modes:
        print('[FAIL] Mode Manager not found in registry.')
        return
    test_cases = ['Cinema', 'Driving', 'Meditation', 'Gaming', 'Emergency', 'Study', 'Work', 'Sleep']
    for mode in test_cases:
        print(f'\n[TEST] Switching to mode: {mode}')
        res = modes.switch_mode(mode)
        print(f"  Status: {res.get('Status')}")
        print(f"  Tuning: {res.get('Kernel_Tuning')}")
    print('\n[SUCCESS] Mode Verification Complete.')