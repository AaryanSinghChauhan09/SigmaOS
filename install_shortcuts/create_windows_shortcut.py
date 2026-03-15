# Generated file: create_windows_shortcut
import os
import sys

def create_windows_shortcut():
    """Native Windows Shortcut Creation via PowerShell."""
    if sys.platform != 'win32':
        print('[SIGMA] Shortcut creation skipped (Target OS: UNIX). Use SigmaOS.sh directly.')
        return
    try:
        import subprocess
        root = os.path.dirname(os.path.abspath(__file__))
        target = os.path.join(root, 'SigmaOS.bat')
        desktop = os.path.join(os.path.expanduser('~'), 'Desktop', 'SigmaOS Sovereign.lnk')
        ps_cmd = f"$s=(New-Object -ComObject WScript.Shell).CreateShortcut('{desktop}');$s.TargetPath='{target}';$s.WorkingDirectory='{root}';$s.Save()"
        subprocess.run(['powershell', '-Command', ps_cmd], check=True)
        print(f'[SIGMA] Sovereign Desktop Shortcut Created: {desktop}')
    except Exception as e:
        print(f'[ERROR] Failed to create shortcut: {e}')