# Σ SIGMAOS SOVEREIGN: NATIVE POWER PERSONALIZER (Zero-Python)
# ==============================================================
# USP: Native Window/Wallpaper management via Win32 SPI.

$code = @"
using System;
using System.Runtime.InteropServices;

namespace SigmaOS.Native {
    public class Personalizer {
        [DllImport("user32.dll", CharSet = CharSet.Auto)]
        static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);

        private const int SPI_SETDESKWALLPAPER = 0x0014;
        private const int SPIF_UPDATEINIFILE = 0x01;
        private const int SPIF_SENDCHANGE = 0x02;

        public static void SetWallpaper(string path) {
            SystemParametersInfo(SPI_SETDESKWALLPAPER, 0, path, SPIF_UPDATEINIFILE | SPIF_SENDCHANGE);
        }
    }
}
"@

Add-Type -TypeDefinition $code

$vibe = $args[0]
if ($vibe -eq "CYBERPUNK") {
    Write-Host "[NATIVE] Applying Cyberpunk Vibe..."
    # Placeholder for actual local asset
    # [SigmaOS.Native.Personalizer]::SetWallpaper("$PSScriptRoot\assets\themes\cyberpunk.jpg")
}
elseif ($vibe -eq "WORK") {
    Write-Host "[NATIVE] Applying Work Vibe (Minimalist)..."
}
else {
    Write-Host "[NATIVE] Default Sovereign Applied."
}
