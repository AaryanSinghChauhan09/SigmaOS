/**
 * =========================================================================
 * Σ SIGMAOS: WIN32 API TYPE DEFINITIONS
 * =========================================================================
 * HWND, HDC, WNDCLASS, MSG, RECT and all Win32 surface types for
 * sigma-kernel32, sigma-user32, sigma-gdi32.
 * =========================================================================
 */
#pragma once
#include "sigma_nt_types.h"

/* -----------------------------------------------------------------------
 * Win32 opaque handle types — represented as tagged u32 indices into
 * sigma-handle-table (NOT raw pointers, for security).
 * ----------------------------------------------------------------------- */
typedef sigma_u32  HWND;    /* Window handle */
typedef sigma_u32  HDC;     /* Device context handle */
typedef sigma_u32  HFONT;   /* Font handle */
typedef sigma_u32  HBRUSH;  /* Brush handle */
typedef sigma_u32  HPEN;    /* Pen handle */
typedef sigma_u32  HBITMAP; /* Bitmap handle */
typedef sigma_u32  HMENU;   /* Menu handle */
typedef sigma_u32  HICON;   /* Icon handle */
typedef sigma_u32  HCURSOR; /* Cursor handle */
typedef sigma_u32  HMODULE; /* Module handle (PE image base) */
typedef sigma_u32  HINSTANCE;
typedef sigma_u32  HKEY;    /* Registry key handle */
typedef sigma_u32  HGLOBAL; /* Global memory handle */
typedef sigma_u32  ATOM;    /* Window class atom */

#define NULL_HWND    ((HWND)0)
#define NULL_HDC     ((HDC)0)
#define HWND_DESKTOP ((HWND)0)

/* -----------------------------------------------------------------------
 * RECT, POINT, SIZE
 * ----------------------------------------------------------------------- */
typedef struct { sigma_s32 left, top, right, bottom; } RECT;
typedef struct { sigma_s32 x, y; }                    POINT;
typedef struct { sigma_s32 cx, cy; }                   SIZE;
typedef RECT*  LPRECT;
typedef POINT* LPPOINT;

/* -----------------------------------------------------------------------
 * Window message (MSG)
 * ----------------------------------------------------------------------- */
typedef sigma_u32 WPARAM;
typedef sigma_s32 LPARAM;
typedef sigma_s32 LRESULT;

typedef struct {
    HWND   hwnd;
    UINT   message;   /* WM_xxx */
    WPARAM wParam;
    LPARAM lParam;
    DWORD  time;
    POINT  pt;
} MSG;
typedef MSG* LPMSG;

/* -----------------------------------------------------------------------
 * Window messages
 * ----------------------------------------------------------------------- */
#define WM_NULL             0x0000
#define WM_CREATE           0x0001
#define WM_DESTROY          0x0002
#define WM_MOVE             0x0003
#define WM_SIZE             0x0005
#define WM_ACTIVATE         0x0006
#define WM_SETFOCUS         0x0007
#define WM_KILLFOCUS        0x0008
#define WM_PAINT            0x000F
#define WM_CLOSE            0x0010
#define WM_QUIT             0x0012
#define WM_KEYDOWN          0x0100
#define WM_KEYUP            0x0101
#define WM_CHAR             0x0102
#define WM_SYSKEYDOWN       0x0104
#define WM_SYSKEYUP         0x0105
#define WM_MOUSEMOVE        0x0200
#define WM_LBUTTONDOWN      0x0201
#define WM_LBUTTONUP        0x0202
#define WM_RBUTTONDOWN      0x0204
#define WM_RBUTTONUP        0x0205
#define WM_MOUSEWHEEL       0x020A
#define WM_USER             0x0400

/* -----------------------------------------------------------------------
 * Window styles
 * ----------------------------------------------------------------------- */
#define WS_OVERLAPPED       0x00000000UL
#define WS_CAPTION          0x00C00000UL
#define WS_SYSMENU          0x00080000UL
#define WS_THICKFRAME       0x00040000UL
#define WS_MINIMIZEBOX      0x00020000UL
#define WS_MAXIMIZEBOX      0x00010000UL
#define WS_OVERLAPPEDWINDOW (WS_OVERLAPPED|WS_CAPTION|WS_SYSMENU|WS_THICKFRAME|WS_MINIMIZEBOX|WS_MAXIMIZEBOX)
#define WS_VISIBLE          0x10000000UL
#define WS_CHILD            0x40000000UL
#define WS_POPUP            0x80000000UL

/* Extended window styles */
#define WS_EX_APPWINDOW     0x00040000UL
#define WS_EX_WINDOWEDGE    0x00000100UL
#define WS_EX_OVERLAPPEDWINDOW (WS_EX_WINDOWEDGE|0x00000200UL)

/* -----------------------------------------------------------------------
 * WNDCLASSEX — window class registration
 * ----------------------------------------------------------------------- */
typedef LRESULT (*WNDPROC)(HWND, UINT, WPARAM, LPARAM);

typedef struct {
    UINT      cbSize;
    UINT      style;
    WNDPROC   lpfnWndProc;
    int       cbClsExtra;
    int       cbWndExtra;
    HINSTANCE hInstance;
    HICON     hIcon;
    HCURSOR   hCursor;
    HBRUSH    hbrBackground;
    LPCSTR    lpszMenuName;
    LPCSTR    lpszClassName;
    HICON     hIconSm;
} WNDCLASSEXA;

/* ShowWindow commands */
#define SW_HIDE             0
#define SW_SHOWNORMAL       1
#define SW_SHOW             5
#define SW_MAXIMIZE         3
#define SW_MINIMIZE         6

/* -----------------------------------------------------------------------
 * PAINTSTRUCT — BeginPaint / EndPaint
 * ----------------------------------------------------------------------- */
typedef struct {
    HDC   hdc;
    BOOL  fErase;
    RECT  rcPaint;
    BOOL  fRestore;
    BOOL  fIncUpdate;
    BYTE  rgbReserved[32];
} PAINTSTRUCT;

/* -----------------------------------------------------------------------
 * CREATESTRUCT — passed to WM_CREATE via lParam
 * ----------------------------------------------------------------------- */
typedef struct {
    LPVOID    lpCreateParams;
    HINSTANCE hInstance;
    HMENU     hMenu;
    HWND      hwndParent;
    int       cy, cx, y, x;
    LONG      style;
    LPCSTR    lpszName;
    LPCSTR    lpszClass;
    DWORD     dwExStyle;
} CREATESTRUCTA;

/* -----------------------------------------------------------------------
 * GetMessage / PeekMessage filter flags
 * ----------------------------------------------------------------------- */
#define PM_NOREMOVE  0x0000
#define PM_REMOVE    0x0001
#define PM_NOYIELD   0x0002

/* -----------------------------------------------------------------------
 * MessageBox flags
 * ----------------------------------------------------------------------- */
#define MB_OK               0x00000000
#define MB_OKCANCEL         0x00000001
#define MB_YESNO            0x00000004
#define MB_ICONERROR        0x00000010
#define MB_ICONWARNING      0x00000030
#define MB_ICONINFORMATION  0x00000040
#define IDOK     1
#define IDCANCEL 2
#define IDYES    6
#define IDNO     7

/* -----------------------------------------------------------------------
 * Standard console handles
 * ----------------------------------------------------------------------- */
#define STD_INPUT_HANDLE  ((DWORD)-10)
#define STD_OUTPUT_HANDLE ((DWORD)-11)
#define STD_ERROR_HANDLE  ((DWORD)-12)

/* -----------------------------------------------------------------------
 * UINT / BOOL / TRUE / FALSE / BOOL macros
 * ----------------------------------------------------------------------- */
typedef sigma_u32 UINT;
typedef sigma_s32 BOOL;
typedef sigma_u32 COLORREF;

#ifndef TRUE
#define TRUE  1
#define FALSE 0
#endif

/* RGB macro for COLORREF */
#define RGB(r,g,b) ((COLORREF)(((BYTE)(r)) | (((DWORD)((BYTE)(g)))<<8) | (((DWORD)((BYTE)(b)))<<16)))
