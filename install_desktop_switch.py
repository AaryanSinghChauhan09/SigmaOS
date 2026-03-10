"""
SigmaOS Desktop Integrator (Context Menu Injection)
===================================================
Injects a direct 'Switch to SigmaOS' button natively into the Windows Right-Click Menu.
This allows the user to boot instances of SigmaOS from literally anywhere on their Desktop
without needing an icon shortcut.
"""

import winreg
import os
import sys

def add_context_menu():
    key_path = r"Software\Classes\Directory\Background\shell\SigmaOS"
    command_path = r"Software\Classes\Directory\Background\shell\SigmaOS\command"
    
    # Get the absolute path of the SigmaOS directory
    root_dir = os.path.abspath(os.path.dirname(__file__))
    
    # The magical command that boots the GUI from any directory without keeping a persistent console
    cmd_string = f'wscript.exe "{root_dir}\\Play_SigmaOS.vbs"'
    
    # We will also create the helper VBScript if it doesn't exist, to ensure silent boot
    vbs_path = os.path.join(root_dir, "Play_SigmaOS.vbs")
    if not os.path.exists(vbs_path):
        with open(vbs_path, "w") as f:
            f.write('Set objShell = WScript.CreateObject("WScript.Shell")\n')
            f.write(f'objShell.Run "cmd /c cd /d ""{root_dir}"" && python sigma_gui.py", 0, False\n')
            
    try:
        # Create main context menu item
        key = winreg.CreateKey(winreg.HKEY_CURRENT_USER, key_path)
        winreg.SetValueEx(key, "MUIVerb", 0, winreg.REG_SZ, "🚀 Switch to SigmaOS Sovereign")
        # Set a cool internal windows icon (A futuristic shield or star)
        winreg.SetValueEx(key, "Icon", 0, winreg.REG_SZ, "imageres.dll,-104")
        winreg.CloseKey(key)
        
        # Create the command execution subroutine
        cmd_key = winreg.CreateKey(winreg.HKEY_CURRENT_USER, command_path)
        winreg.SetValueEx(cmd_key, "", 0, winreg.REG_SZ, cmd_string)
        winreg.CloseKey(cmd_key)
        
        print("==================================================")
        print("✅ SUCCESS: SIGMA OS INJECTED INTO WINDOWS KERNEL")
        print("==================================================")
        print("You can now right-click anywhere on your Windows")
        print("Desktop background to instantly 'Switch to SigmaOS'.")
        return True
    except Exception as e:
        print(f"Error: {e}")
        return False

if __name__ == "__main__":
    add_context_menu()
