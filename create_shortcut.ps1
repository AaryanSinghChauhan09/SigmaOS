$WshShell = New-Object -ComObject WScript.Shell
$DesktopPath = [System.IO.Path]::Combine($env:USERPROFILE, "Desktop")
$Shortcut = $WshShell.CreateShortcut("$DesktopPath\SigmaOS.lnk")
$Shortcut.TargetPath = "py.exe"
$Shortcut.Arguments = "C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\sigma_gui.py"
$Shortcut.WorkingDirectory = "C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
$Shortcut.Description = "Boot into SigmaOS Sovereign v2.0"
$Shortcut.Save()
Write-Host "SigmaOS Desktop Shortcut Created Successfully!"
