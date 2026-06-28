/**
 * =========================================================================
 * Σ SIGMAOS: NT NATIVE API DECLARATIONS (sigma-ntdll)
 * =========================================================================
 * NtXxx / ZwXxx / RtlXxx function prototypes.
 * All are implemented in runtime/compat/win32/sigma_ntdll.cpp.
 * =========================================================================
 */
#pragma once
#include "sigma_nt_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* -----------------------------------------------------------------------
 * File I/O
 * ----------------------------------------------------------------------- */

/* NtCreateFile — open or create a file/device
 * FileAttributes: FILE_ATTRIBUTE_NORMAL=0x80
 * ShareAccess:    FILE_SHARE_READ=1, FILE_SHARE_WRITE=2
 * CreateDisposition: FILE_OPEN=1, FILE_CREATE=2, FILE_OPEN_IF=3,
 *                    FILE_OVERWRITE_IF=5
 * CreateOptions:  FILE_SYNCHRONOUS_IO_NONALERT=0x20
 */
NTSTATUS NtCreateFile(
    PHANDLE            FileHandle,
    ACCESS_MASK        DesiredAccess,
    POBJECT_ATTRIBUTES ObjectAttributes,
    PIO_STATUS_BLOCK   IoStatusBlock,
    PLARGE_INTEGER     AllocationSize,
    ULONG              FileAttributes,
    ULONG              ShareAccess,
    ULONG              CreateDisposition,
    ULONG              CreateOptions,
    PVOID              EaBuffer,
    ULONG              EaLength
);

NTSTATUS NtReadFile(
    HANDLE           FileHandle,
    HANDLE           Event,
    PVOID            ApcRoutine,
    PVOID            ApcContext,
    PIO_STATUS_BLOCK IoStatusBlock,
    PVOID            Buffer,
    ULONG            Length,
    PLARGE_INTEGER   ByteOffset,
    PULONG           Key
);

NTSTATUS NtWriteFile(
    HANDLE           FileHandle,
    HANDLE           Event,
    PVOID            ApcRoutine,
    PVOID            ApcContext,
    PIO_STATUS_BLOCK IoStatusBlock,
    const PVOID      Buffer,
    ULONG            Length,
    PLARGE_INTEGER   ByteOffset,
    PULONG           Key
);

NTSTATUS NtClose(HANDLE Handle);

NTSTATUS NtQueryInformationFile(
    HANDLE                 FileHandle,
    PIO_STATUS_BLOCK       IoStatusBlock,
    PVOID                  FileInformation,
    ULONG                  Length,
    ULONG                  FileInformationClass
);

/* -----------------------------------------------------------------------
 * Virtual memory
 * ----------------------------------------------------------------------- */
NTSTATUS NtAllocateVirtualMemory(
    HANDLE  ProcessHandle,
    PVOID*  BaseAddress,
    ULONG   ZeroBits,
    PULONG  RegionSize,
    ULONG   AllocationType,   /* MEM_COMMIT | MEM_RESERVE */
    ULONG   Protect           /* PAGE_READWRITE etc. */
);

NTSTATUS NtFreeVirtualMemory(
    HANDLE  ProcessHandle,
    PVOID*  BaseAddress,
    PULONG  RegionSize,
    ULONG   FreeType          /* MEM_RELEASE | MEM_DECOMMIT */
);

NTSTATUS NtProtectVirtualMemory(
    HANDLE  ProcessHandle,
    PVOID*  BaseAddress,
    PULONG  RegionSize,
    ULONG   NewProtect,
    PULONG  OldProtect
);

/* -----------------------------------------------------------------------
 * Process & thread
 * ----------------------------------------------------------------------- */
NTSTATUS NtCreateProcess(
    PHANDLE            ProcessHandle,
    ACCESS_MASK        DesiredAccess,
    POBJECT_ATTRIBUTES ObjectAttributes,
    HANDLE             ParentProcess,
    BOOL               InheritObjectTable,
    HANDLE             SectionHandle,
    HANDLE             DebugPort,
    HANDLE             ExceptionPort
);

NTSTATUS NtCreateThread(
    PHANDLE            ThreadHandle,
    ACCESS_MASK        DesiredAccess,
    POBJECT_ATTRIBUTES ObjectAttributes,
    HANDLE             ProcessHandle,
    PVOID*             ClientId,
    PVOID              ThreadContext,
    PVOID              InitialTeb,
    BOOL               CreateSuspended
);

NTSTATUS NtTerminateProcess(HANDLE ProcessHandle, NTSTATUS ExitStatus);
NTSTATUS NtTerminateThread(HANDLE ThreadHandle, NTSTATUS ExitStatus);

NTSTATUS NtQueryInformationProcess(
    HANDLE ProcessHandle,
    ULONG  ProcessInformationClass,
    PVOID  ProcessInformation,
    ULONG  ProcessInformationLength,
    PULONG ReturnLength
);

NTSTATUS NtQueryInformationThread(
    HANDLE ThreadHandle,
    ULONG  ThreadInformationClass,
    PVOID  ThreadInformation,
    ULONG  ThreadInformationLength,
    PULONG ReturnLength
);

NTSTATUS NtSetInformationThread(
    HANDLE ThreadHandle,
    ULONG  ThreadInformationClass,
    PVOID  ThreadInformation,
    ULONG  ThreadInformationLength
);

/* -----------------------------------------------------------------------
 * Synchronization
 * ----------------------------------------------------------------------- */
NTSTATUS NtCreateMutant(
    PHANDLE            MutantHandle,
    ACCESS_MASK        DesiredAccess,
    POBJECT_ATTRIBUTES ObjectAttributes,
    BOOL               InitialOwner
);

NTSTATUS NtReleaseMutant(HANDLE MutantHandle, PULONG PreviousCount);

NTSTATUS NtCreateEvent(
    PHANDLE            EventHandle,
    ACCESS_MASK        DesiredAccess,
    POBJECT_ATTRIBUTES ObjectAttributes,
    ULONG              EventType,    /* 0=NotificationEvent, 1=SynchronizationEvent */
    BOOL               InitialState
);

NTSTATUS NtSetEvent(HANDLE EventHandle, PULONG PreviousState);
NTSTATUS NtResetEvent(HANDLE EventHandle, PULONG PreviousState);

NTSTATUS NtWaitForSingleObject(
    HANDLE         Handle,
    BOOL           Alertable,
    PLARGE_INTEGER Timeout    /* NULL = infinite */
);

NTSTATUS NtWaitForMultipleObjects(
    ULONG           Count,
    const HANDLE*   Handles,
    ULONG           WaitType,   /* 0=WaitAny, 1=WaitAll */
    BOOL            Alertable,
    PLARGE_INTEGER  Timeout
);

NTSTATUS NtDelayExecution(BOOL Alertable, PLARGE_INTEGER DelayInterval);

/* -----------------------------------------------------------------------
 * Registry
 * ----------------------------------------------------------------------- */
NTSTATUS NtCreateKey(
    PHANDLE            KeyHandle,
    ACCESS_MASK        DesiredAccess,
    POBJECT_ATTRIBUTES ObjectAttributes,
    ULONG              TitleIndex,
    PUNICODE_STRING    Class,
    ULONG              CreateOptions,
    PULONG             Disposition
);

NTSTATUS NtOpenKey(
    PHANDLE            KeyHandle,
    ACCESS_MASK        DesiredAccess,
    POBJECT_ATTRIBUTES ObjectAttributes
);

NTSTATUS NtQueryValueKey(
    HANDLE          KeyHandle,
    PUNICODE_STRING ValueName,
    ULONG           KeyValueInformationClass,
    PVOID           KeyValueInformation,
    ULONG           Length,
    PULONG          ResultLength
);

NTSTATUS NtSetValueKey(
    HANDLE          KeyHandle,
    PUNICODE_STRING ValueName,
    ULONG           TitleIndex,
    ULONG           Type,           /* REG_SZ=1, REG_DWORD=4, REG_BINARY=3 */
    PVOID           Data,
    ULONG           DataSize
);

NTSTATUS NtDeleteKey(HANDLE KeyHandle);
NTSTATUS NtDeleteValueKey(HANDLE KeyHandle, PUNICODE_STRING ValueName);

/* Registry value types */
#define REG_NONE        0
#define REG_SZ          1
#define REG_EXPAND_SZ   2
#define REG_BINARY      3
#define REG_DWORD       4
#define REG_QWORD       11

/* -----------------------------------------------------------------------
 * System information
 * ----------------------------------------------------------------------- */
NTSTATUS NtQuerySystemInformation(
    ULONG  SystemInformationClass,
    PVOID  SystemInformation,
    ULONG  SystemInformationLength,
    PULONG ReturnLength
);

/* SystemBasicInformation = 0 */
typedef struct {
    ULONG     Reserved;
    ULONG     TimerResolution;
    ULONG     PageSize;
    ULONG     NumberOfPhysicalPages;
    ULONG     LowestPhysicalPageNumber;
    ULONG     HighestPhysicalPageNumber;
    ULONG     AllocationGranularity;
    ULONG_PTR MinimumUserModeAddress;
    ULONG_PTR MaximumUserModeAddress;
    ULONG_PTR ActiveProcessorsAffinityMask;
    ULONG     NumberOfProcessors;
} SYSTEM_BASIC_INFORMATION;

/* -----------------------------------------------------------------------
 * Time
 * ----------------------------------------------------------------------- */
NTSTATUS NtQuerySystemTime(PLARGE_INTEGER SystemTime);
NTSTATUS NtQueryPerformanceCounter(
    PLARGE_INTEGER PerformanceCounter,
    PLARGE_INTEGER PerformanceFrequency
);

/* -----------------------------------------------------------------------
 * Rtl heap (NT's allocator layer)
 * ----------------------------------------------------------------------- */
PVOID  RtlAllocateHeap(HANDLE HeapHandle, ULONG Flags, sigma_usize Size);
BOOL   RtlFreeHeap(HANDLE HeapHandle, ULONG Flags, PVOID BaseAddress);
PVOID  RtlReAllocateHeap(HANDLE HeapHandle, ULONG Flags, PVOID Mem, sigma_usize Size);
sigma_usize RtlSizeHeap(HANDLE HeapHandle, ULONG Flags, PVOID MemoryPointer);

/* -----------------------------------------------------------------------
 * String utilities
 * ----------------------------------------------------------------------- */
void   RtlInitUnicodeString(PUNICODE_STRING DestinationString, PCWSTR SourceString);
NTSTATUS RtlUnicodeStringToAnsiString(
    void* DestinationString,   /* PANSI_STRING */
    PUNICODE_STRING SourceString,
    BOOL AllocateDestinationString
);
void   RtlFreeUnicodeString(PUNICODE_STRING UnicodeString);
LONG   RtlCompareUnicodeString(
    PUNICODE_STRING String1,
    PUNICODE_STRING String2,
    BOOL CaseInSensitive
);

#ifdef __cplusplus
} /* extern "C" */
#endif
