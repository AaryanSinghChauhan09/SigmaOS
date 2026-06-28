/**
 * =========================================================================
 * Σ SIGMAOS: NT NATIVE API TYPE DEFINITIONS
 * =========================================================================
 * NTSTATUS codes, UNICODE_STRING, OBJECT_ATTRIBUTES, PEB, TEB, and all
 * other NT-layer types required by sigma-ntdll and sigma-wine.
 * Does NOT depend on any Microsoft headers.
 * =========================================================================
 */
#pragma once
#include "../sigma_kernel_types.h"
#include "sigma_pe_types.h"

/* -----------------------------------------------------------------------
 * NTSTATUS — NT return code (32-bit signed)
 * ----------------------------------------------------------------------- */
typedef sigma_s32 NTSTATUS;
typedef NTSTATUS* PNTSTATUS;

#define NT_SUCCESS(s)   ((NTSTATUS)(s) >= 0)
#define NT_FAILURE(s)   ((NTSTATUS)(s) <  0)

/* Common NTSTATUS values */
#define STATUS_SUCCESS                   ((NTSTATUS)0x00000000)
#define STATUS_PENDING                   ((NTSTATUS)0x00000103)
#define STATUS_TIMEOUT                   ((NTSTATUS)0x00000102)
#define STATUS_UNSUCCESSFUL              ((NTSTATUS)0xC0000001)
#define STATUS_NOT_IMPLEMENTED           ((NTSTATUS)0xC0000002)
#define STATUS_INVALID_PARAMETER         ((NTSTATUS)0xC000000D)
#define STATUS_ACCESS_DENIED             ((NTSTATUS)0xC0000022)
#define STATUS_BUFFER_TOO_SMALL          ((NTSTATUS)0xC0000023)
#define STATUS_OBJECT_NAME_NOT_FOUND     ((NTSTATUS)0xC0000034)
#define STATUS_OBJECT_PATH_NOT_FOUND     ((NTSTATUS)0xC000003A)
#define STATUS_END_OF_FILE               ((NTSTATUS)0xC0000011)
#define STATUS_NO_MEMORY                 ((NTSTATUS)0xC0000017)
#define STATUS_INSUFFICIENT_RESOURCES    ((NTSTATUS)0xC000009A)

/* -----------------------------------------------------------------------
 * Basic NT types
 * ----------------------------------------------------------------------- */
typedef sigma_u16  WCHAR;     /* UTF-16LE code unit */
typedef WCHAR*     PWSTR;
typedef const WCHAR* PCWSTR;
typedef PVOID      HANDLE;
typedef HANDLE*    PHANDLE;
typedef sigma_u32  ACCESS_MASK;
typedef sigma_u64  LARGE_INTEGER;
typedef LARGE_INTEGER* PLARGE_INTEGER;
typedef sigma_u32  ULONG_PTR;

#define INVALID_HANDLE_VALUE ((HANDLE)(sigma_u64)-1)
#define NULL_HANDLE          ((HANDLE)0)

/* Standard access rights */
#define GENERIC_READ    0x80000000UL
#define GENERIC_WRITE   0x40000000UL
#define GENERIC_EXECUTE 0x20000000UL
#define GENERIC_ALL     0x10000000UL

/* -----------------------------------------------------------------------
 * UNICODE_STRING — NT's primary string type (UTF-16, not NUL-terminated)
 * ----------------------------------------------------------------------- */
typedef struct {
    USHORT  Length;        /* byte length of Buffer (not including NUL) */
    USHORT  MaximumLength; /* byte capacity of Buffer */
    PWSTR   Buffer;        /* UTF-16LE characters */
} UNICODE_STRING;
typedef UNICODE_STRING* PUNICODE_STRING;

/* -----------------------------------------------------------------------
 * OBJECT_ATTRIBUTES — passed to NtCreateFile, NtOpenKey, etc.
 * ----------------------------------------------------------------------- */
typedef struct {
    ULONG           Length;             /* sizeof(OBJECT_ATTRIBUTES) = 48 */
    HANDLE          RootDirectory;      /* optional base handle */
    PUNICODE_STRING ObjectName;         /* object name relative to root */
    ULONG           Attributes;         /* OBJ_CASE_INSENSITIVE etc. */
    PVOID           SecurityDescriptor;
    PVOID           SecurityQualityOfService;
} OBJECT_ATTRIBUTES;
typedef OBJECT_ATTRIBUTES* POBJECT_ATTRIBUTES;

#define OBJ_CASE_INSENSITIVE 0x00000040UL
#define OBJ_INHERIT          0x00000002UL

/* InitializeObjectAttributes macro */
#define InitializeObjectAttributes(p, n, a, r, s) \
    do { (p)->Length = sizeof(OBJECT_ATTRIBUTES); \
         (p)->RootDirectory = (r);                \
         (p)->Attributes = (a);                   \
         (p)->ObjectName = (n);                   \
         (p)->SecurityDescriptor = (s);           \
         (p)->SecurityQualityOfService = NULL; } while(0)

/* -----------------------------------------------------------------------
 * IO_STATUS_BLOCK — result from NtReadFile / NtWriteFile
 * ----------------------------------------------------------------------- */
typedef struct {
    union {
        NTSTATUS Status;
        PVOID    Pointer;
    };
    ULONG_PTR Information; /* bytes transferred */
} IO_STATUS_BLOCK;
typedef IO_STATUS_BLOCK* PIO_STATUS_BLOCK;

/* -----------------------------------------------------------------------
 * PEB — Process Environment Block (at gs:[0x60] on x86-64)
 * Only fields actually probed by CRT / Win32 startup code.
 * ----------------------------------------------------------------------- */
typedef struct _LDR_DATA_TABLE_ENTRY {
    /* Doubly-linked lists for load order, memory order, init order */
    PVOID InLoadOrderLinks[2];
    PVOID InMemoryOrderLinks[2];
    PVOID InInitializationOrderLinks[2];
    PVOID DllBase;
    PVOID EntryPoint;
    ULONG SizeOfImage;
    UNICODE_STRING FullDllName;
    UNICODE_STRING BaseDllName;
} LDR_DATA_TABLE_ENTRY;

typedef struct _PEB_LDR_DATA {
    ULONG  Length;
    BYTE   Initialized;
    HANDLE SsHandle;
    PVOID  InLoadOrderModuleList[2];   /* LIST_ENTRY head */
    PVOID  InMemoryOrderModuleList[2];
    PVOID  InInitializationOrderModuleList[2];
} PEB_LDR_DATA;

typedef struct _RTL_USER_PROCESS_PARAMETERS {
    ULONG  MaximumLength;
    ULONG  Length;
    ULONG  Flags;
    ULONG  DebugFlags;
    HANDLE ConsoleHandle;
    ULONG  ConsoleFlags;
    HANDLE StandardInput;
    HANDLE StandardOutput;
    HANDLE StandardError;
    UNICODE_STRING CurrentDirectory;
    UNICODE_STRING DllPath;
    UNICODE_STRING ImagePathName;
    UNICODE_STRING CommandLine;
} RTL_USER_PROCESS_PARAMETERS;

/* Minimal PEB — fields at their correct Windows x86-64 offsets */
typedef struct _SIGMA_PEB {
    BYTE    InheritedAddressSpace;      /* +0x000 */
    BYTE    ReadImageFileExecOptions;   /* +0x001 */
    BYTE    BeingDebugged;              /* +0x002 — always 0 */
    BYTE    BitField;                   /* +0x003 */
    PVOID   Mutant;                     /* +0x008 */
    PVOID   ImageBaseAddress;           /* +0x010 */
    PEB_LDR_DATA* Ldr;                 /* +0x018 — module list */
    RTL_USER_PROCESS_PARAMETERS* ProcessParameters; /* +0x020 */
    PVOID   SubSystemData;             /* +0x028 */
    PVOID   ProcessHeap;               /* +0x030 — default heap */
    PVOID   FastPebLock;               /* +0x038 */
    BYTE    _pad1[0x60];               /* padding to OSVersion fields */
    ULONG   OSMajorVersion;            /* +0x0A4 — report 10 */
    ULONG   OSMinorVersion;            /* +0x0A8 — report 0 */
    USHORT  OSBuildNumber;             /* +0x0AC — report 19041 */
    USHORT  OSCSDVersion;              /* +0x0AE */
    ULONG   OSPlatformId;              /* +0x0B0 — VER_PLATFORM_WIN32_NT=2 */
} SIGMA_PEB;

/* -----------------------------------------------------------------------
 * TEB — Thread Environment Block (at gs:[0x00] / fs:[0x18] on x86-64)
 * ----------------------------------------------------------------------- */
typedef struct _SIGMA_TEB {
    /* NT_TIB — first 56 bytes */
    PVOID   ExceptionList;             /* +0x000 — SEH chain head */
    PVOID   StackBase;                 /* +0x008 */
    PVOID   StackLimit;                /* +0x010 */
    PVOID   SubSystemTib;             /* +0x018 */
    PVOID   FiberData;                 /* +0x020 */
    PVOID   ArbitraryUserPointer;      /* +0x028 */
    PVOID   Self;                      /* +0x030 — points to TEB itself */
    /* Extended TEB */
    PVOID   EnvironmentPointer;        /* +0x038 */
    ULONG   ProcessId;                 /* +0x040 */
    ULONG   ThreadId;                  /* +0x048 */
    PVOID   ActiveRpcHandle;           /* +0x050 */
    PVOID   ThreadLocalStoragePointer; /* +0x058 — TLS array */
    SIGMA_PEB* ProcessEnvironmentBlock; /* +0x060 — PEB pointer */
    ULONG   LastErrorValue;            /* +0x068 — GetLastError() */
    BYTE    _pad2[0x1400];
} SIGMA_TEB;

/* -----------------------------------------------------------------------
 * VirtualAlloc / VirtualProtect constants
 * ----------------------------------------------------------------------- */
#define MEM_COMMIT      0x00001000
#define MEM_RESERVE     0x00002000
#define MEM_DECOMMIT    0x00004000
#define MEM_RELEASE     0x00008000

#define PAGE_NOACCESS   0x01
#define PAGE_READONLY   0x02
#define PAGE_READWRITE  0x04
#define PAGE_EXECUTE    0x10
#define PAGE_EXECUTE_READ       0x20
#define PAGE_EXECUTE_READWRITE  0x40
