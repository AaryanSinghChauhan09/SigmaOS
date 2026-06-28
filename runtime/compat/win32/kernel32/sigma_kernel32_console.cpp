/**
 * =========================================================================
 * Σ SIGMAOS: kernel32.dll — Console I/O  — Stage W1
 * =========================================================================
 * GetStdHandle, WriteConsoleA/W, ReadConsoleA, AllocConsole, SetConsoleTitleA
 * These are the first kernel32 functions needed to run any CLI Win32 app.
 *
 * All output is routed to sigma_sys_write() on the underlying sigma-vfs fd.
 * Input is routed to sigma_sys_read() on stdin.
 * =========================================================================
 */
#include "../../../../include/sigma_kernel_types.h"
#include "../../../../include/sigma_log.h"
#include "../../../../include/compat/sigma_nt_types.h"
#include "../../../../include/compat/sigma_win32_types.h"
#include "../../../../include/compat/sigma_nt_syscalls.h"

extern "C" {
    sigma_s64 sigma_sys_read(sigma_u32 fd, void* buf, sigma_usize count);
    sigma_s64 sigma_sys_write(sigma_u32 fd, const void* buf, sigma_usize count);
}

/* -----------------------------------------------------------------------
 * Minimal UTF-16 → UTF-8 converter for WriteConsoleW
 * Only handles BMP (U+0000 – U+FFFF), which covers all Indian scripts.
 * ----------------------------------------------------------------------- */
static sigma_usize utf16_to_utf8(const WCHAR* src, sigma_u32 src_chars,
                                   char* dst, sigma_usize dst_cap) {
    sigma_usize out = 0;
    for (sigma_u32 i = 0; i < src_chars && out + 4 < dst_cap; i++) {
        sigma_u32 cp = (sigma_u32)src[i];
        if (cp < 0x80) {
            dst[out++] = (char)cp;
        } else if (cp < 0x800) {
            dst[out++] = (char)(0xC0 | (cp >> 6));
            dst[out++] = (char)(0x80 | (cp & 0x3F));
        } else {
            dst[out++] = (char)(0xE0 | (cp >> 12));
            dst[out++] = (char)(0x80 | ((cp >> 6) & 0x3F));
            dst[out++] = (char)(0x80 | (cp & 0x3F));
        }
    }
    dst[out] = '\0';
    return out;
}

/* -----------------------------------------------------------------------
 * GetStdHandle — returns a pseudo-handle for stdin/stdout/stderr
 * ----------------------------------------------------------------------- */
HANDLE GetStdHandle(DWORD nStdHandle) {
    if (nStdHandle == (DWORD)(sigma_u64)STD_INPUT_HANDLE)  return (HANDLE)(sigma_u64)0;
    if (nStdHandle == (DWORD)(sigma_u64)STD_OUTPUT_HANDLE) return (HANDLE)(sigma_u64)1;
    if (nStdHandle == (DWORD)(sigma_u64)STD_ERROR_HANDLE)  return (HANDLE)(sigma_u64)2;
    return INVALID_HANDLE_VALUE;
}

/* -----------------------------------------------------------------------
 * WriteConsoleA — write ASCII/ANSI text to console
 * ----------------------------------------------------------------------- */
BOOL WriteConsoleA(HANDLE hConsoleOutput, const void* lpBuffer,
                   DWORD nNumberOfCharsToWrite,
                   DWORD* lpNumberOfCharsWritten,
                   PVOID lpReserved)
{
    (void)lpReserved;
    sigma_u32 fd = (sigma_u32)(sigma_u64)hConsoleOutput;
    if (fd > 2) fd = 1; /* default to stdout */
    sigma_s64 n = sigma_sys_write(fd, lpBuffer, (sigma_usize)nNumberOfCharsToWrite);
    if (n < 0) {
        if (lpNumberOfCharsWritten) *lpNumberOfCharsWritten = 0;
        return FALSE;
    }
    if (lpNumberOfCharsWritten) *lpNumberOfCharsWritten = (DWORD)n;
    return TRUE;
}

/* -----------------------------------------------------------------------
 * WriteConsoleW — write UTF-16 text; we convert to UTF-8 for sigma-vfs
 * ----------------------------------------------------------------------- */
BOOL WriteConsoleW(HANDLE hConsoleOutput, const void* lpBuffer,
                   DWORD nNumberOfCharsToWrite,
                   DWORD* lpNumberOfCharsWritten,
                   PVOID lpReserved)
{
    (void)lpReserved;
    char utf8[4096];
    sigma_usize bytes = utf16_to_utf8(
        (const WCHAR*)lpBuffer, nNumberOfCharsToWrite, utf8, sizeof(utf8));
    sigma_u32 fd = (sigma_u32)(sigma_u64)hConsoleOutput;
    if (fd > 2) fd = 1;
    sigma_s64 n = sigma_sys_write(fd, utf8, bytes);
    if (n < 0) {
        if (lpNumberOfCharsWritten) *lpNumberOfCharsWritten = 0;
        return FALSE;
    }
    /* Report chars (not bytes) written */
    if (lpNumberOfCharsWritten) *lpNumberOfCharsWritten = nNumberOfCharsToWrite;
    return TRUE;
}

/* -----------------------------------------------------------------------
 * ReadConsoleA — read from stdin
 * ----------------------------------------------------------------------- */
BOOL ReadConsoleA(HANDLE hConsoleInput, void* lpBuffer,
                  DWORD nNumberOfCharsToRead,
                  DWORD* lpNumberOfCharsRead,
                  PVOID lpReserved)
{
    (void)lpReserved;
    sigma_u32 fd = (sigma_u32)(sigma_u64)hConsoleInput;
    sigma_s64 n  = sigma_sys_read(fd, lpBuffer, (sigma_usize)nNumberOfCharsToRead);
    if (n <= 0) {
        if (lpNumberOfCharsRead) *lpNumberOfCharsRead = 0;
        return FALSE;
    }
    if (lpNumberOfCharsRead) *lpNumberOfCharsRead = (DWORD)n;
    return TRUE;
}

/* -----------------------------------------------------------------------
 * WriteFile — used by many CRT implementations for console output
 * ----------------------------------------------------------------------- */
BOOL WriteFile(HANDLE hFile, const void* lpBuffer, DWORD nBytesToWrite,
               DWORD* lpBytesWritten, PVOID lpOverlapped)
{
    (void)lpOverlapped;
    /* Route through NtWriteFile */
    IO_STATUS_BLOCK iosb = {};
    NTSTATUS s = NtWriteFile(hFile, SIGMA_NULL, SIGMA_NULL, SIGMA_NULL,
                              &iosb, (PVOID)lpBuffer, nBytesToWrite,
                              SIGMA_NULL, SIGMA_NULL);
    if (lpBytesWritten) *lpBytesWritten = NT_SUCCESS(s) ? (DWORD)iosb.Information : 0;
    return NT_SUCCESS(s) ? TRUE : FALSE;
}

/* -----------------------------------------------------------------------
 * ReadFile
 * ----------------------------------------------------------------------- */
BOOL ReadFile(HANDLE hFile, void* lpBuffer, DWORD nBytesToRead,
              DWORD* lpBytesRead, PVOID lpOverlapped)
{
    (void)lpOverlapped;
    IO_STATUS_BLOCK iosb = {};
    NTSTATUS s = NtReadFile(hFile, SIGMA_NULL, SIGMA_NULL, SIGMA_NULL,
                             &iosb, lpBuffer, nBytesToRead,
                             SIGMA_NULL, SIGMA_NULL);
    if (lpBytesRead) *lpBytesRead = NT_SUCCESS(s) ? (DWORD)iosb.Information : 0;
    return (NT_SUCCESS(s) || s == STATUS_END_OF_FILE) ? TRUE : FALSE;
}

/* -----------------------------------------------------------------------
 * Console attribute stubs (ANSI escape codes via sigma-vfs)
 * ----------------------------------------------------------------------- */
BOOL SetConsoleTextAttribute(HANDLE hConsoleOutput, WORD wAttributes) {
    (void)hConsoleOutput; (void)wAttributes;
    /* TODO: map Windows color attributes to ANSI escape codes */
    return TRUE;
}

BOOL GetConsoleScreenBufferInfo(HANDLE hConsoleOutput, void* lpConsoleScreenBufferInfo) {
    (void)hConsoleOutput;
    /* Fill with 80×25 default — enough for most CLI apps */
    if (lpConsoleScreenBufferInfo) {
        sigma_u16* p = (sigma_u16*)lpConsoleScreenBufferInfo;
        p[0] = 80;  /* dwSize.X */
        p[1] = 25;  /* dwSize.Y */
        p[2] = 0;   /* dwCursorPosition.X */
        p[3] = 0;   /* dwCursorPosition.Y */
        p[4] = 0x07; /* wAttributes (white on black) */
    }
    return TRUE;
}

BOOL SetConsoleTitleA(const char* lpConsoleTitle) {
    sigma_log_info("[kernel32] SetConsoleTitleA(\"%s\")", lpConsoleTitle);
    /* TODO: update Zenith window title */
    return TRUE;
}

BOOL AllocConsole(void) {
    sigma_log_info("[kernel32] AllocConsole() — already available via sigma-vfs");
    return TRUE;
}

BOOL FreeConsole(void) { return TRUE; }

} /* extern "C" */
