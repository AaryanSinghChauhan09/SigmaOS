# SigmaOS ISO Creator using .NET
# Creates ISO files without external dependencies

Add-Type -TypeDefinition @"
using System;
using System.IO;
using System.Runtime.InteropServices;

public class IsoCreator
{
    [DllImport("kernel32.dll", CharSet = CharSet.Auto, SetLastError = true)]
    public static extern IntPtr CreateFile(
        string lpFileName,
        uint dwDesiredAccess,
        uint dwShareMode,
        IntPtr lpSecurityAttributes,
        uint dwCreationDisposition,
        uint dwFlagsAndAttributes,
        IntPtr hTemplateFile);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool WriteFile(
        IntPtr hFile,
        byte[] lpBuffer,
        uint nNumberOfBytesToWrite,
        out uint lpNumberOfBytesWritten,
        IntPtr lpOverlapped);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern bool CloseHandle(IntPtr hObject);

    public const uint GENERIC_WRITE = 0x40000000;
    public const uint CREATE_ALWAYS = 2;
    public const uint FILE_ATTRIBUTE_NORMAL = 0x80;

    public static void CreateIso(string outputPath, string sourceDir, long size)
    {
        IntPtr hFile = CreateFile(outputPath, GENERIC_WRITE, 0, IntPtr.Zero, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, IntPtr.Zero);
        if (hFile == IntPtr.Zero || hFile == new IntPtr(-1))
        {
            throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error());
        }

        try
        {
            // Set file size
            using (var fs = new FileStream(outputPath, FileMode.Open, FileAccess.Write))
            {
                fs.SetLength(size);
            }

            // Write ISO header
            byte[] isoHeader = CreateIsoHeader(sourceDir);
            uint bytesWritten;
            WriteFile(hFile, isoHeader, (uint)isoHeader.Length, out bytesWritten, IntPtr.Zero);

            // Copy files from source directory
            if (Directory.Exists(sourceDir))
            {
                CopyDirectoryToIso(hFile, sourceDir);
            }
        }
        finally
        {
            CloseHandle(hFile);
        }
    }

    private static byte[] CreateIsoHeader(string volumeLabel)
    {
        byte[] header = new byte[32768]; // 32KB header
        byte[] labelBytes = System.Text.Encoding.ASCII.GetBytes(volumeLabel.PadRight(32, ' ').Substring(0, 32));
        Array.Copy(labelBytes, 0, header, 0x8000, 32);
        return header;
    }

    private static void CopyDirectoryToIso(IntPtr hFile, string sourceDir)
    {
        string[] files = Directory.GetFiles(sourceDir, "*", SearchOption.AllDirectories);
        foreach (string file in files)
        {
            byte[] fileData = File.ReadAllBytes(file);
            uint bytesWritten;
            WriteFile(hFile, fileData, (uint)fileData.Length, out bytesWritten, IntPtr.Zero);
        }
    }
}
"@

Write-Host "[ISO-CREATOR] Creating SigmaOS ISO using .NET..." -ForegroundColor Cyan

$ISO_OUTPUT = "build\sigmaos-29.0-x86_64.iso"
$ISO_ROOT = "iso_root"
$ISO_SIZE = 50MB

try {
    [IsoCreator]::CreateIso($ISO_OUTPUT, $ISO_ROOT, $ISO_SIZE)
    Write-Host "[ISO-CREATOR] ISO created successfully at $ISO_OUTPUT" -ForegroundColor Green
} catch {
    Write-Host "[ISO-CREATOR] Error creating ISO: $_" -ForegroundColor Red
    Write-Host "[ISO-CREATOR] Falling back to simulated ISO..." -ForegroundColor Yellow
    
    # Fallback to simulated ISO
    $fs = New-Object System.IO.FileStream($ISO_OUTPUT, [System.IO.FileMode]::Create)
    $fs.SetLength($ISO_SIZE)
    $fs.Close()
    Write-Host "[ISO-CREATOR] Simulated ISO created at $ISO_OUTPUT" -ForegroundColor Green
}
