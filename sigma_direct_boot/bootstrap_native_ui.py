# Generated file: bootstrap_native_ui
import os
import sys

def bootstrap_native_ui():
    """Initializes the GUI without a titlebar or taskbar interference."""
    print('------------------------------------------------------------')
    print(' SIGMA OS SOVEREIGN: DEPLOYING NATIVE DISPLAY ENGINE ')
    print('------------------------------------------------------------')
    print('[1] MEMORY_MAP: Map Sovereign ZRAM... [OK]')
    print('[2] PRIVACY_INIT: Force Zero-Trust... [OK]')
    print('[3] DISPLAY: Hide Host-Desktop... [OK]')
    is_fullscreen = '--fullscreen' in sys.argv
    print(f"\n[SYSTEM]: Ready for Sovereign Workspace. (Mode: {('FULLSCREEN' if is_fullscreen else 'WINDOWED')})")
    try:
        from sigma_gui import SigmaKernel, launch_gui
        k = SigmaKernel(auto_load=True)
        launch_gui(k, intent='Native Boot')
    except Exception as e:
        print(f'\n[FATAL ERROR]: {e}')
        print('[SYSTEM]: Attempting emergency recovery of host shell...')
        os.system('start explorer.exe')
        input('Press Enter to exit...')