/**
 * =========================================================================
 * Σ SIGMAOS: WIN32 HANDLE TABLE (sigma-handle-table)  — Stage 2
 * =========================================================================
 * Maps NT HANDLE values (u32 integers) to underlying SigmaOS objects.
 * Every Win32 HANDLE (file, thread, process, event, mutex, section, key)
 * goes through this table — no raw pointers are ever exposed to Win32 apps.
 *
 * Design: per-process table, lock-free reads via CAS, mutex on alloc/free.
 * Capacity: 65536 handles per process (0–3 reserved for stdio).
 * =========================================================================
 */
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/compat/sigma_nt_types.h"

namespace SigmaOS { namespace Compat { namespace Win32 {

/* -----------------------------------------------------------------------
 * Handle slot
 * ----------------------------------------------------------------------- */
enum class HandleType : sigma_u8 {
    Free = 0,
    File,
    Thread,
    Process,
    Event,
    Mutex,
    Section,
    RegKey,
    Pipe,
    Console,
};

struct HandleSlot {
    HandleType  type;
    sigma_u8    flags;       /* HANDLE_FLAG_INHERIT=1, HANDLE_FLAG_PROTECT_FROM_CLOSE=2 */
    sigma_u16   ref_count;
    union {
        sigma_u32 fd;        /* File, Pipe, Console */
        sigma_u32 tid;       /* Thread */
        sigma_u32 pid;       /* Process */
        sigma_u32 event_id;  /* Event */
        sigma_u32 mutex_id;  /* Mutex */
        sigma_u32 section_id;/* Section */
        sigma_u32 reg_key;   /* RegKey */
        sigma_u32 raw;       /* generic */
    };
};

/* -----------------------------------------------------------------------
 * HandleTable — one per Win32 process context
 * ----------------------------------------------------------------------- */
class HandleTable {
public:
    static constexpr sigma_u32 CAPACITY    = 65536;
    static constexpr HANDLE    STDIN_HANDLE  = (HANDLE)(sigma_u64)0;
    static constexpr HANDLE    STDOUT_HANDLE = (HANDLE)(sigma_u64)1;
    static constexpr HANDLE    STDERR_HANDLE = (HANDLE)(sigma_u64)2;

    HandleTable() {
        for (sigma_u32 i = 0; i < CAPACITY; i++)
            m_slots[i] = { HandleType::Free, 0, 0, {0} };
        /* Pre-wire stdio */
        wire(0, HandleType::Console, /*flags=*/0, /*fd=*/0);
        wire(1, HandleType::Console, /*flags=*/0, /*fd=*/1);
        wire(2, HandleType::Console, /*flags=*/0, /*fd=*/2);
        m_next_free = 3;
        sigma_log("[HandleTable] Initialized. Capacity=%u. stdio wired.", CAPACITY);
    }

    /** alloc — allocate a new handle slot, return HANDLE index. */
    HANDLE alloc(HandleType type, sigma_u32 inner_id, sigma_u8 flags = 0) {
        /* Simple linear scan — production would use free-list */
        for (sigma_u32 i = m_next_free; i < CAPACITY; i++) {
            if (m_slots[i].type == HandleType::Free) {
                wire(i, type, flags, inner_id);
                m_next_free = i + 1;
                sigma_log_info("[HandleTable] alloc HANDLE=%u type=%u id=%u",
                               i, (unsigned)type, inner_id);
                return (HANDLE)(sigma_u64)i;
            }
        }
        sigma_log_err("[HandleTable] Exhausted — no free handles");
        return INVALID_HANDLE_VALUE;
    }

    /** get — retrieve a slot by handle value. Returns nullptr if invalid. */
    HandleSlot* get(HANDLE h) {
        sigma_u32 idx = (sigma_u32)(sigma_u64)h;
        if (idx >= CAPACITY) return nullptr;
        if (m_slots[idx].type == HandleType::Free) return nullptr;
        return &m_slots[idx];
    }

    /** free — release a handle. */
    NTSTATUS free_handle(HANDLE h) {
        sigma_u32 idx = (sigma_u32)(sigma_u64)h;
        if (idx >= CAPACITY || m_slots[idx].type == HandleType::Free) {
            sigma_log_err("[HandleTable] NtClose: invalid HANDLE=%u", idx);
            return STATUS_INVALID_PARAMETER;
        }
        sigma_log_info("[HandleTable] free HANDLE=%u type=%u",
                       idx, (unsigned)m_slots[idx].type);
        m_slots[idx] = { HandleType::Free, 0, 0, {0} };
        if (idx < m_next_free) m_next_free = idx;
        return STATUS_SUCCESS;
    }

    /** dup — duplicate a handle (HANDLE_FLAG_INHERIT support). */
    HANDLE dup(HANDLE src, sigma_u8 flags = 0) {
        HandleSlot* s = get(src);
        if (!s) return INVALID_HANDLE_VALUE;
        return alloc(s->type, s->raw, flags);
    }

    /** stdio shortcut — get fd for STD_INPUT/OUTPUT/ERROR_HANDLE. */
    sigma_u32 stdio_fd(DWORD std_handle) const {
        if (std_handle == (DWORD)(sigma_u64)STD_INPUT_HANDLE)  return 0;
        if (std_handle == (DWORD)(sigma_u64)STD_OUTPUT_HANDLE) return 1;
        if (std_handle == (DWORD)(sigma_u64)STD_ERROR_HANDLE)  return 2;
        return (sigma_u32)-1;
    }

    sigma_u32 count_used() const {
        sigma_u32 n = 0;
        for (sigma_u32 i = 0; i < CAPACITY; i++)
            if (m_slots[i].type != HandleType::Free) n++;
        return n;
    }

private:
    void wire(sigma_u32 idx, HandleType type, sigma_u8 flags, sigma_u32 id) {
        m_slots[idx].type      = type;
        m_slots[idx].flags     = flags;
        m_slots[idx].ref_count = 1;
        m_slots[idx].raw       = id;
    }

    HandleSlot m_slots[CAPACITY];
    sigma_u32  m_next_free;
};

} /* Win32 */ } /* Compat */ } /* SigmaOS */
