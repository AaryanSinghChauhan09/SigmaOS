function Fix-Includes {
    param([string]$Path, [int]$UpLevels)
    $Content = Get-Content $Path -Raw
    $Prefix = if ($UpLevels -gt 0) { "../" * $UpLevels } else { "./" }
    
    $NewContent = [regex]::Replace($Content, '#include\s+"\./include/', "#include `"${Prefix}include/")
    if ($Content -cne $NewContent) {
        Set-Content -Path $Path -Value $NewContent -NoNewline -Encoding UTF8
        Write-Host "Fixed: $Path"
    }
}

if (Test-Path "userland") {
    Get-ChildItem -Path userland -Filter *.cpp | ForEach-Object { Fix-Includes $_.FullName 1 }
}
if (Test-Path "tools") {
    Get-ChildItem -Path tools -Filter *.cpp | ForEach-Object { Fix-Includes $_.FullName 1 }
}

if (Test-Path "kernel") {
    Get-ChildItem -Path kernel -Recurse -Include *.cpp, *.h | ForEach-Object {
        $Rel = $_.FullName.Substring((Get-Location).Path.Length + 1)
        $RelTrim = $Rel -replace "^kernel[\\/]", ""
        $Levels = ($RelTrim -split "[\\/]").Count
        Fix-Includes $_.FullName $Levels
    }
}

if (Test-Path "include") {
    Get-ChildItem -Path include -Recurse -Include *.h, *.hpp | ForEach-Object {
        $Content = Get-Content $_.FullName -Raw
        $Rel = $_.FullName.Substring((Get-Location).Path.Length + 1)
        $RelTrim = $Rel -replace "^include[\\/]", ""
        $Depth = ($RelTrim -split "[\\/]").Count - 1
        $Prefix = if ($Depth -gt 0) { "../" * $Depth } else { "./" }
        
        $NewContent = [regex]::Replace($Content, '#include\s+"\./include/', "#include `"$Prefix")
        if ($Content -cne $NewContent) {
            Set-Content -Path $_.FullName -Value $NewContent -NoNewline -Encoding UTF8
            Write-Host "Fixed Include: $_.FullName"
        }
    }
}
