Set objShell = WScript.CreateObject("WScript.Shell")
strPath = Left(WScript.ScriptFullName, InStrRev(WScript.ScriptFullName, "\") - 1)
objShell.CurrentDirectory = strPath
objShell.Run "cmd /c py boot.py", 0, False
