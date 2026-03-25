$WshShell = New-Object -ComObject WScript.Shell
$DesktopPath = [System.IO.Path]::Combine($env:USERPROFILE, "Desktop")
$WorkingDir = (Get-Item -Path $PSScriptRoot).FullName

$Shortcut = $WshShell.CreateShortcut("$DesktopPath\SigmaOS Sovereign.lnk")
$Shortcut.TargetPath = "py.exe"
$Shortcut.Arguments = "boot.py"
$Shortcut.WorkingDirectory = $WorkingDir
$Shortcut.IconLocation = "$WorkingDir\assets\icon.ico" # If it exists
$Shortcut.Description = "Boot into SigmaOS Sovereign v2.0 (APEX)"
$Shortcut.Save()

Write-Host "[OK] SigmaOS Desktop Shortcut Created at: $DesktopPath\SigmaOS Sovereign.lnk" -ForegroundColor Green
