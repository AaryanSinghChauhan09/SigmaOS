/**
 * =========================================================================
 * Σ SIGMAOS: NT NATIVE API TRANSLATOR (sigma-ntdll)  — Stage 2
 * =========================================================================
 * Implements all NtXxx / ZwXxx / RtlXxx functions declared in
 * include/compat/sigma_nt_syscalls.h by translating to sigma-syscall ABI.
 *
 * Each function maps NT semantics → sigma primitive:
 *   NtReadFile      → sigma_sys_read
 *   NtWriteFile     → sigma_sys_write
 *   NtCreateFile    → sigma_sys_open
 *   NtClose         → sigma_sys_close + handle table free
 *   NtAllocVirtMem  → sigma_sys_mmap
 *   NtFreeVirtMem   → sigma_sys_munmap
 *   NtCreateThread  → sigma_sys_thread_create
 *   NtCreateEvent   → sigma_event_create
 *   ...
 *
 * Status: core I/O + memory + sync implemented as stubs with correct
 *         return semantics. Full VMM and threading pending Phase 0.
 * =========================================================================
 */
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/compat/sigma_nt_types.h"
#include "../../../include/compat/sigma_nt_syscalls.h"
#include "sigma_handle_table.cpp"  /* inline for now; will be separate TU */

/* Forward-declare sigma-syscall primitives (kernel ABI) */
extern "C" {
    sigma_s64 sigma_sys_read(sigma_u32 fd, void* buf, sigma_usize count);
    sigma_s64 sigma_sys_write(sigma_u32 fd, const void* buf, sigma_usize count);
    sigma_s32 sigma_sys_open(const char* path, sigma_u32 flags);
    sigma_s32 sigma_sys_close(sigma_u32 fd);
    void*     sigma_sys_mmap(void* addr, sigma_usize len, sigma_u32 prot,
                              sigma_u32 flags, sigma_s32 fd, sigma_u64 off);
    sigma_s32 sigma_sys_munmap(void* addr, sigma_usize len);
    sigma_s32 sigma_sys_mprotect(void* addr, sigma_usize len, sigma_u32 prot);
    sigma_u32 sigma_sys_getpid(void);
    sigma_u64 sigma_sys_uptime_ms(void);
    sigma_s32 sigma_sys_nanosleep(sigma_u64 ns);
    sigma_s32 sigma_event_create(sigma_u32 type, sigma_u32 initial);
    sigma_s32 sigma_event_set(sigma_u32 id);
    sigma_s32 sigma_event_reset(sigma_u32 id);
    sigma_s32 sigma_wait_single(sigma_u32 id, sigma_u64 timeout_ns);
    sigma_s32 sigma_mutex_create(sigma_u32 initial_owner);
    sigma_s32 sigma_mutex_release(sigma_u32 id);
}

/* -----------------------------------------------------------------------
 * Process-local handle table (one per Win32 process context)
 * ----------------------------------------------------------------------- */
static SigmaOS::Compat::Win32::HandleTable g_handles;

/* -----------------------------------------------------------------------
 * NT → sigma permission mapping helpers
 * ----------------------------------------------------------------------- */
static sigma_u32 nt_page_to_prot(ULONG nt) {
    switch (nt) {
        case PAGE_NOACCESS:          return 0;
        case PAGE_READONLY:          return 4;          /* PROT_READ */
        case PAGE_READWRITE:         return 4 | 2;      /* PROT_READ|WRITE */
        case PAGE_EXECUTE:           return 1;          /* PROT_EXEC */
        case PAGE_EXECUTE_READ:      return 4 | 1;
        case PAGE_EXECUTE_READWRITE: return 4 | 2 | 1;
        default:                     return 4 | 2;
    }
}

/* -----------------------------------------------------------------------
 * File I/O
 * ----------------------------------------------------------------------- */
extern "C" NTSTATUS NtCreateFile(
    PHANDLE FileHandle, ACCESS_MASK Access,
    POBJECT_ATTRIBUTES ObjAttr, PIO_STATUS_BLOCK IoSb,
    PLARGE_INTEGER AllocSize, ULONG FileAttr,
    ULONG ShareAccess, ULONG Disposition,
    ULONG Options, PVOID EaBuf, ULONG EaLen)
{
    (void)AllocSize; (void)FileAttr; (void)ShareAccess;
    (void)Options; (void)EaBuf; (void)EaLen;
    if (!FileHandle || !ObjAttr) return STATUS_INVALID_PARAMETER;

    /* Extract path from UNICODE_STRING — convert UTF-16 → ASCII (best-effort) */
    char path[512] = {0};
    if (ObjAttr->ObjectName && ObjAttr->ObjectName->Buffer) {
        USHORT byte_len = ObjAttr->ObjectName->Length;
        USHORT chars = (USHORT)(byte_len / 2);
        if (chars > 511) chars = 511;
        for (USHORT i = 0; i < chars; i++)
            path[i] = (char)(ObjAttr->ObjectName->Buffer[i] & 0x7F);
        path[chars] = '\0';
        /* Strip NT path prefix: \??\C:\... → /sigma/wine/c/... */
        /* TODO: full NT path normalizer */
    }

    sigma_u32 flags = (Access & GENERIC_WRITE) ? 2 : 0;
    sigma_s32 fd    = sigma_sys_open(path, flags);
    if (fd < 0) {
        if (IoSb) { IoSb->Status = STATUS_OBJECT_NAME_NOT_FOUND; IoSb->Information = 0; }
        sigma_log_err("[ntdll] NtCreateFile(\"%s\"): open failed fd=%d", path, fd);
        return STATUS_OBJECT_NAME_NOT_FOUND;
    }

    *FileHandle = g_handles.alloc(
        SigmaOS::Compat::Win32::HandleType::File, (sigma_u32)fd);
    if (IoSb) { IoSb->Status = STATUS_SUCCESS; IoSb->Information = 1; /* FILE_OPENED */ }
    sigma_log_info("[ntdll] NtCreateFile(\"%s\") → HANDLE=%llu fd=%d",
                   path, (unsigned long long)(sigma_u64)*FileHandle, fd);
    return STATUS_SUCCESS;
}

extern "C" NTSTATUS NtReadFile(
    HANDLE FileHandle, HANDLE Event, PVOID ApcRoutine, PVOID ApcContext,
    PIO_STATUS_BLOCK IoSb, PVOID Buffer, ULONG Length,
    PLARGE_INTEGER ByteOffset, PULONG Key)
{
    (void)Event; (void)ApcRoutine; (void)ApcContext; (void)ByteOffset; (void)Key;
    auto* slot = g_handles.get(FileHandle);
    if (!slot) return STATUS_INVALID_PARAMETER;
    sigma_s64 n = sigma_sys_read(slot->fd, Buffer, (sigma_usize)Length);
    if (n < 0) {
        if (IoSb) { IoSb->Status = STATUS_UNSUCCESSFUL; IoSb->Information = 0; }
        return STATUS_UNSUCCESSFUL;
    }
    if (IoSb) { IoSb->Status = STATUS_SUCCESS; IoSb->Information = (ULONG_PTR)n; }
    return (n == 0) ? STATUS_END_OF_FILE : STATUS_SUCCESS;
}

extern "C" NTSTATUS NtWriteFile(
    HANDLE FileHandle, HANDLE Event, PVOID ApcRoutine, PVOID ApcContext,
    PIO_STATUS_BLOCK IoSb, const PVOID Buffer, ULONG Length,
    PLARGE_INTEGER ByteOffset, PULONG Key)
{
    (void)Event; (void)ApcRoutine; (void)ApcContext; (void)ByteOffset; (void)Key;
    auto* slot = g_handles.get(FileHandle);
    if (!slot) return STATUS_INVALID_PARAMETER;
    sigma_s64 n = sigma_sys_write(slot->fd, Buffer, (sigma_usize)Length);
    if (n < 0) {
        if (IoSb) { IoSb->Status = STATUS_UNSUCCESSFUL; IoSb->Information = 0; }
        return STATUS_UNSUCCESSFUL;
    }
    if (IoSb) { IoSb->Status = STATUS_SUCCESS; IoSb->Information = (ULONG_PTR)n; }
    return STATUS_SUCCESS;
}

extern "C" NTSTATUS NtClose(HANDLE Handle) {
    auto* slot = g_handles.get(Handle);
    if (!slot) return STATUS_INVALID_PARAMETER;
    if (slot->type == SigmaOS::Compat::Win32::HandleType::File ||
        slot->type == SigmaOS::Compat::Win32::HandleType::Console)
        sigma_sys_close(slot->fd);
    return g_handles.free_handle(Handle);
}

/* -----------------------------------------------------------------------
 * Virtual memory
 * ----------------------------------------------------------------------- */
extern "C" NTSTATUS NtAllocateVirtualMemory(
    HANDLE Process, PVOID* BaseAddress, ULONG ZeroBits,
    PULONG RegionSize, ULONG AllocationType, ULONG Protect)
{
    (void)Process; (void)ZeroBits;
    sigma_u32 prot = nt_page_to_prot(Protect);
    sigma_usize size = RegionSize ? (sigma_usize)*RegionSize : 0x1000;
    void* addr = *BaseAddress; /* hint; may be NULL */
    void* mem  = sigma_sys_mmap(addr, size, prot, 0x22 /*MAP_PRIVATE|MAP_ANON*/, -1, 0);
    if (!mem) return STATUS_NO_MEMORY;
    if (BaseAddress) *BaseAddress = mem;
    if (RegionSize)  *RegionSize  = (ULONG)size;
    sigma_log_info("[ntdll] NtAllocateVirtualMemory size=0x%lx → %p", (unsigned long)size, mem);
    return STATUS_SUCCESS;
}

extern "C" NTSTATUS NtFreeVirtualMemory(
    HANDLE Process, PVOID* BaseAddress, PULONG RegionSize, ULONG FreeType)
{
    (void)Process; (void)FreeType;
    if (!BaseAddress || !*BaseAddress) return STATUS_INVALID_PARAMETER;
    sigma_usize size = RegionSize ? (sigma_usize)*RegionSize : 0;
    sigma_sys_munmap(*BaseAddress, size);
    return STATUS_SUCCESS;
}

extern "C" NTSTATUS NtProtectVirtualMemory(
    HANDLE Process, PVOID* BaseAddress, PULONG RegionSize,
    ULONG NewProtect, PULONG OldProtect)
{
    (void)Process;
    if (OldProtect) *OldProtect = PAGE_READWRITE; /* conservative */
    sigma_u32 prot = nt_page_to_prot(NewProtect);
    sigma_usize size = RegionSize ? (sigma_usize)*RegionSize : 0x1000;
    sigma_sys_mprotect(*BaseAddress, size, prot);
    return STATUS_SUCCESS;
}

/* -----------------------------------------------------------------------
 * Synchronization
 * ----------------------------------------------------------------------- */
extern "C" NTSTATUS NtCreateEvent(
    PHANDLE EventHandle, ACCESS_MASK Access,
    POBJECT_ATTRIBUTES ObjAttr, ULONG EventType, BOOL InitialState)
{
    (void)Access; (void)ObjAttr;
    sigma_s32 id = sigma_event_create((sigma_u32)EventType, (sigma_u32)InitialState);
    if (id < 0) return STATUS_INSUFFICIENT_RESOURCES;
    *EventHandle = g_handles.alloc(
        SigmaOS::Compat::Win32::HandleType::Event, (sigma_u32)id);
    return STATUS_SUCCESS;
}

extern "C" NTSTATUS NtSetEvent(HANDLE EventHandle, PULONG PreviousState) {
    (void)PreviousState;
    auto* slot = g_handles.get(EventHandle);
    if (!slot) return STATUS_INVALID_PARAMETER;
    return (sigma_event_set(slot->event_id) == 0) ? STATUS_SUCCESS : STATUS_UNSUCCESSFUL;
}

extern "C" NTSTATUS NtResetEvent(HANDLE EventHandle, PULONG PreviousState) {
    (void)PreviousState;
    auto* slot = g_handles.get(EventHandle);
    if (!slot) return STATUS_INVALID_PARAMETER;
    return (sigma_event_reset(slot->event_id) == 0) ? STATUS_SUCCESS : STATUS_UNSUCCESSFUL;
}

extern "C" NTSTATUS NtWaitForSingleObject(
    HANDLE Handle, BOOL Alertable, PLARGE_INTEGER Timeout)
{
    (void)Alertable;
    auto* slot = g_handles.get(Handle);
    if (!slot) return STATUS_INVALID_PARAMETER;
    sigma_u64 timeout_ns = Timeout ? (sigma_u64)(*Timeout) * 100 : (sigma_u64)-1;
    sigma_s32 r = sigma_wait_single(slot->raw, timeout_ns);
    if (r == 0) return STATUS_SUCCESS;
    if (r == -1) return STATUS_TIMEOUT;
    return STATUS_UNSUCCESSFUL;
}

extern "C" NTSTATUS NtDelayExecution(BOOL Alertable, PLARGE_INTEGER DelayInterval) {
    (void)Alertable;
    if (!DelayInterval) return STATUS_INVALID_PARAMETER;
    /* NT delay is in 100-nanosecond units, negative = relative */
    sigma_u64 ns = (sigma_u64)(-(*DelayInterval)) * 100ULL;
    sigma_sys_nanosleep(ns);
    return STATUS_SUCCESS;
}

extern "C" NTSTATUS NtCreateMutant(
    PHANDLE MutantHandle, ACCESS_MASK Access,
    POBJECT_ATTRIBUTES ObjAttr, BOOL InitialOwner)
{
    (void)Access; (void)ObjAttr;
    sigma_s32 id = sigma_mutex_create((sigma_u32)InitialOwner);
    if (id < 0) return STATUS_INSUFFICIENT_RESOURCES;
    *MutantHandle = g_handles.alloc(
        SigmaOS::Compat::Win32::HandleType::Mutex, (sigma_u32)id);
    return STATUS_SUCCESS;
}

/* -----------------------------------------------------------------------
 * System information
 * ----------------------------------------------------------------------- */
extern "C" NTSTATUS NtQuerySystemInformation(
    ULONG Class, PVOID Info, ULONG Len, PULONG RetLen)
{
    if (Class == 0 /* SystemBasicInformation */) {
        if (Len < sizeof(SYSTEM_BASIC_INFORMATION)) return STATUS_BUFFER_TOO_SMALL;
        SYSTEM_BASIC_INFORMATION* sbi = (SYSTEM_BASIC_INFORMATION*)Info;
        sbi->PageSize                  = 4096;
        sbi->NumberOfPhysicalPages     = 262144; /* 1 GB fake */
        sbi->LowestPhysicalPageNumber  = 1;
        sbi->HighestPhysicalPageNumber = 262143;
        sbi->AllocationGranularity     = 65536;
        sbi->MinimumUserModeAddress    = 0x10000;
        sbi->MaximumUserModeAddress    = 0x7FFFFFFFFFFFull;
        sbi->NumberOfProcessors        = 4; /* fake */
        if (RetLen) *RetLen = sizeof(SYSTEM_BASIC_INFORMATION);
        return STATUS_SUCCESS;
    }
    sigma_log_info("[ntdll] NtQuerySystemInformation class=%u — stub", Class);
    return STATUS_NOT_IMPLEMENTED;
}

/* -----------------------------------------------------------------------
 * Time / performance
 * ----------------------------------------------------------------------- */
extern "C" NTSTATUS NtQueryPerformanceCounter(
    PLARGE_INTEGER Counter, PLARGE_INTEGER Freq)
{
    sigma_u64 tsc = 0;
    __asm__ volatile("rdtsc" : "=A"(tsc));
    if (Counter) *Counter = (LARGE_INTEGER)tsc;
    if (Freq)    *Freq    = (LARGE_INTEGER)3000000000ULL; /* 3 GHz assumed */
    return STATUS_SUCCESS;
}

extern "C" NTSTATUS NtQuerySystemTime(PLARGE_INTEGER SysTime) {
    if (!SysTime) return STATUS_INVALID_PARAMETER;
    /* NT FILETIME: 100-ns intervals since Jan 1, 1601
     * Approximate: uptime_ms * 10000 + epoch_offset */
    sigma_u64 ms = sigma_sys_uptime_ms();
    *SysTime = (LARGE_INTEGER)(ms * 10000ULL + 116444736000000000ULL);
    return STATUS_SUCCESS;
}

/* -----------------------------------------------------------------------
 * Rtl heap — thin wrapper over sigma_slab
 * ----------------------------------------------------------------------- */
extern "C" {
    void* sigma_slab_alloc(sigma_usize size);
    void  sigma_slab_free(void* ptr);
    void* sigma_slab_realloc(void* ptr, sigma_usize size);
    sigma_usize sigma_slab_size(void* ptr);
}

extern "C" PVOID RtlAllocateHeap(HANDLE Heap, ULONG Flags, sigma_usize Size) {
    (void)Heap; (void)Flags;
    return sigma_slab_alloc(Size);
}
extern "C" BOOL RtlFreeHeap(HANDLE Heap, ULONG Flags, PVOID Ptr) {
    (void)Heap; (void)Flags;
    sigma_slab_free(Ptr);
    return TRUE;
}
extern "C" PVOID RtlReAllocateHeap(HANDLE Heap, ULONG Flags, PVOID Mem, sigma_usize Size) {
    (void)Heap; (void)Flags;
    return sigma_slab_realloc(Mem, Size);
}
extern "C" sigma_usize RtlSizeHeap(HANDLE Heap, ULONG Flags, PVOID Ptr) {
    (void)Heap; (void)Flags;
    return sigma_slab_size(Ptr);
}

/* -----------------------------------------------------------------------
 * String utilities
 * ----------------------------------------------------------------------- */
extern "C" void RtlInitUnicodeString(PUNICODE_STRING Dest, PCWSTR Source) {
    if (!Dest) return;
    Dest->Buffer = (PWSTR)Source;
    if (!Source) { Dest->Length = Dest->MaximumLength = 0; return; }
    USHORT len = 0;
    while (Source[len]) len++;
    Dest->Length        = (USHORT)(len * 2);
    Dest->MaximumLength = (USHORT)((len + 1) * 2);
}

extern "C" void RtlFreeUnicodeString(PUNICODE_STRING Str) {
    if (Str && Str->Buffer) {
        sigma_slab_free(Str->Buffer);
        Str->Buffer = SIGMA_NULL;
        Str->Length = Str->MaximumLength = 0;
    }
}
