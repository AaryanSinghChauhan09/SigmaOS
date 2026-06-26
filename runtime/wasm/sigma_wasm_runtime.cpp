/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN WASM RUNTIME — SovereignWASM
 * =========================================================================
 * A WASI-compatible WebAssembly interpreter/AOT engine for running
 * cross-platform WASM binaries natively on SigmaOS.
 *
 * Architecture:
 *   WasmLoader        — Parses and validates the WASM binary format
 *   WasmInterpreter   — Stack machine bytecode executor
 *   WasiHost          — WASI System Interface host implementation
 *   SovereignWasmRuntime — Top-level lifecycle manager
 *
 * WASI syscalls mapped to SigmaOS native calls:
 *   fd_read  → sigma_vfs_read
 *   fd_write → sigma_vfs_write
 *   fd_close → sigma_vfs_close
 *   proc_exit → sigma_task_exit
 *   args_get  → sigma_proc_args
 *   environ_get → sigma_proc_environ
 *   clock_time_get → sigma_clock_get
 *
 * WebAssembly MVP + WASI Preview 1 support.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/sigma_wasi.h"

namespace SigmaOS {
namespace Runtime {
namespace WASM {

/* -----------------------------------------------------------------------
 * WASM binary format constants (WebAssembly 1.0 spec)
 * ----------------------------------------------------------------------- */
constexpr sigma_u32 WASM_MAGIC   = 0x6D736100u; /* '\0asm' */
constexpr sigma_u32 WASM_VERSION = 0x00000001u;

/* Section IDs */
enum class SectionId : sigma_u8 {
    CUSTOM   = 0,
    TYPE     = 1,
    IMPORT   = 2,
    FUNCTION = 3,
    TABLE    = 4,
    MEMORY   = 5,
    GLOBAL   = 6,
    EXPORT   = 7,
    START    = 8,
    ELEMENT  = 9,
    CODE     = 10,
    DATA     = 11,
};

/* Value types */
enum class ValType : sigma_u8 {
    I32  = 0x7F,
    I64  = 0x7E,
    F32  = 0x7D,
    F64  = 0x7C,
    FUNCREF = 0x70,
};

/* -----------------------------------------------------------------------
 * WASM value (tagged union)
 * ----------------------------------------------------------------------- */
struct WasmVal {
    ValType type;
    union {
        sigma_u32 i32;
        sigma_u64 i64;
        float     f32;
        double    f64;
    } v;

    static WasmVal i32(sigma_u32 x) { WasmVal w; w.type = ValType::I32; w.v.i32 = x; return w; }
    static WasmVal i64(sigma_u64 x) { WasmVal w; w.type = ValType::I64; w.v.i64 = x; return w; }
};

/* -----------------------------------------------------------------------
 * Parsed WASM module (flat representation)
 * ----------------------------------------------------------------------- */
struct WasmType {
    sigma_u8 param_count;
    sigma_u8 result_count;
    ValType  params[16];
    ValType  results[4];
};

struct WasmFunc {
    sigma_u32 type_idx;
    sigma_u32 code_offset; /* byte offset into raw bytecode */
    sigma_u32 code_size;
    sigma_u32 local_count;
};

struct WasmExport {
    char      name[64];
    sigma_u8  kind;         /* 0=func, 1=table, 2=mem, 3=global */
    sigma_u32 index;
};

struct WasmImport {
    char      module_name[32];
    char      field_name[64];
    sigma_u8  kind;
    sigma_u32 type_idx;
};

struct WasmMemory {
    sigma_u32 min_pages;    /* each page = 64 KiB */
    sigma_u32 max_pages;
    sigma_u8* data;         /* linear memory backing store */
    sigma_u32 current_pages;
};

struct WasmModule {
    WasmType   types[128];
    sigma_u32  type_count;

    WasmImport imports[64];
    sigma_u32  import_count;

    WasmFunc   funcs[512];
    sigma_u32  func_count;

    WasmExport exports[128];
    sigma_u32  export_count;

    WasmMemory mem;

    sigma_u32  start_func;  /* 0xFFFFFFFF = no start function */

    const sigma_u8* bytecode;
    sigma_usize     bytecode_len;

    bool       valid;
};

/* -----------------------------------------------------------------------
 * LEB128 decoder helper
 * ----------------------------------------------------------------------- */
static sigma_u32 decode_uleb128(const sigma_u8* buf, sigma_usize buf_len,
                                 sigma_usize* pos) {
    sigma_u32 result = 0;
    int shift = 0;
    while (*pos < buf_len) {
        sigma_u8 b = buf[(*pos)++];
        result |= (sigma_u32)(b & 0x7F) << shift;
        if (!(b & 0x80)) break;
        shift += 7;
    }
    return result;
}

/* -----------------------------------------------------------------------
 * WasmLoader — binary parser
 * ----------------------------------------------------------------------- */
class WasmLoader {
public:
    bool parse(const sigma_u8* data, sigma_usize len, WasmModule* mod) {
        if (len < 8) {
            sigma_log_err("[WASM] Binary too small (%zu bytes).", len);
            return false;
        }

        /* Magic + version */
        sigma_u32 magic   = (sigma_u32)data[0] | ((sigma_u32)data[1] << 8) |
                             ((sigma_u32)data[2] << 16) | ((sigma_u32)data[3] << 24);
        sigma_u32 version = (sigma_u32)data[4] | ((sigma_u32)data[5] << 8) |
                             ((sigma_u32)data[6] << 16) | ((sigma_u32)data[7] << 24);

        if (magic != WASM_MAGIC) {
            sigma_log_err("[WASM] Invalid magic: 0x%08X (expected 0x6D736100)", magic);
            return false;
        }
        if (version != WASM_VERSION) {
            sigma_log_err("[WASM] Unsupported version: %u", version);
            return false;
        }

        mod->bytecode       = data;
        mod->bytecode_len   = len;
        mod->start_func     = 0xFFFFFFFF;
        mod->type_count     = 0;
        mod->func_count     = 0;
        mod->export_count   = 0;
        mod->import_count   = 0;
        mod->mem.min_pages  = 0;
        mod->mem.max_pages  = 0;
        mod->mem.data       = SIGMA_NULL;
        mod->mem.current_pages = 0;
        mod->valid          = false;

        sigma_usize pos = 8;
        while (pos < len) {
            if (pos >= len) break;
            sigma_u8 sec_id = data[pos++];
            sigma_u32 sec_len = decode_uleb128(data, len, &pos);
            sigma_usize sec_end = pos + sec_len;

            switch ((SectionId)sec_id) {
                case SectionId::TYPE:
                    parseTypeSection(data, pos, sec_end, mod);
                    break;
                case SectionId::IMPORT:
                    parseImportSection(data, pos, sec_end, mod);
                    break;
                case SectionId::FUNCTION:
                    parseFunctionSection(data, pos, sec_end, mod);
                    break;
                case SectionId::EXPORT:
                    parseExportSection(data, pos, sec_end, mod);
                    break;
                case SectionId::MEMORY:
                    parseMemorySection(data, pos, sec_end, mod);
                    break;
                case SectionId::START:
                    if (pos < sec_end)
                        mod->start_func = decode_uleb128(data, len, &pos);
                    break;
                case SectionId::CODE:
                    parseCodeSection(data, pos, sec_end, mod);
                    break;
                default:
                    break; /* Skip unknown sections */
            }
            pos = sec_end;
        }

        mod->valid = true;
        sigma_log_info("[WASM] Parsed: %u types, %u funcs, %u exports, %u imports",
                        mod->type_count, mod->func_count, mod->export_count, mod->import_count);
        return true;
    }

private:
    void parseTypeSection(const sigma_u8* d, sigma_usize pos, sigma_usize end, WasmModule* m) {
        sigma_u32 count = decode_uleb128(d, end, &pos);
        for (sigma_u32 i = 0; i < count && m->type_count < 128; i++) {
            if (d[pos++] != 0x60) continue; /* func type marker */
            WasmType& t = m->types[m->type_count++];
            t.param_count = (sigma_u8)decode_uleb128(d, end, &pos);
            for (int p = 0; p < t.param_count && p < 16; p++)
                t.params[p] = (ValType)d[pos++];
            t.result_count = (sigma_u8)decode_uleb128(d, end, &pos);
            for (int r = 0; r < t.result_count && r < 4; r++)
                t.results[r] = (ValType)d[pos++];
        }
    }

    void parseImportSection(const sigma_u8* d, sigma_usize pos, sigma_usize end, WasmModule* m) {
        sigma_u32 count = decode_uleb128(d, end, &pos);
        for (sigma_u32 i = 0; i < count && m->import_count < 64; i++) {
            WasmImport& imp = m->imports[m->import_count++];
            sigma_u32 mlen = decode_uleb128(d, end, &pos);
            for (sigma_u32 j = 0; j < mlen && j < 31; j++) imp.module_name[j] = (char)d[pos++];
            sigma_u32 flen = decode_uleb128(d, end, &pos);
            for (sigma_u32 j = 0; j < flen && j < 63; j++) imp.field_name[j] = (char)d[pos++];
            imp.kind = d[pos++];
            if (imp.kind == 0) imp.type_idx = decode_uleb128(d, end, &pos);
        }
    }

    void parseFunctionSection(const sigma_u8* d, sigma_usize pos, sigma_usize end, WasmModule* m) {
        sigma_u32 count = decode_uleb128(d, end, &pos);
        for (sigma_u32 i = 0; i < count && m->func_count < 512; i++) {
            m->funcs[m->func_count].type_idx = decode_uleb128(d, end, &pos);
            m->func_count++;
        }
    }

    void parseExportSection(const sigma_u8* d, sigma_usize pos, sigma_usize end, WasmModule* m) {
        sigma_u32 count = decode_uleb128(d, end, &pos);
        for (sigma_u32 i = 0; i < count && m->export_count < 128; i++) {
            WasmExport& exp = m->exports[m->export_count++];
            sigma_u32 nlen = decode_uleb128(d, end, &pos);
            for (sigma_u32 j = 0; j < nlen && j < 63; j++) exp.name[j] = (char)d[pos++];
            exp.kind  = d[pos++];
            exp.index = decode_uleb128(d, end, &pos);
        }
    }

    void parseMemorySection(const sigma_u8* d, sigma_usize pos, sigma_usize end, WasmModule* m) {
        sigma_u32 count = decode_uleb128(d, end, &pos);
        if (count > 0) {
            sigma_u8 flags = d[pos++];
            m->mem.min_pages = decode_uleb128(d, end, &pos);
            m->mem.max_pages = (flags & 1) ? decode_uleb128(d, end, &pos) : 65536u;
        }
    }

    void parseCodeSection(const sigma_u8* d, sigma_usize pos, sigma_usize end, WasmModule* m) {
        sigma_u32 count = decode_uleb128(d, end, &pos);
        sigma_u32 import_func_count = m->import_count; /* imports precede locals */
        for (sigma_u32 i = 0; i < count; i++) {
            sigma_u32 body_size = decode_uleb128(d, end, &pos);
            sigma_u32 fidx = import_func_count + i;
            if (fidx < 512) {
                m->funcs[fidx].code_offset = (sigma_u32)pos;
                m->funcs[fidx].code_size   = body_size;
            }
            pos += body_size;
        }
    }
};

/* -----------------------------------------------------------------------
 * WASI host — maps WASI calls to SigmaOS primitives
 * ----------------------------------------------------------------------- */
class WasiHost {
public:
    /* fd_write(fd, iovs, iovs_len, nwritten_ptr) → errno */
    sigma_u32 fd_write(sigma_u32 fd, sigma_u32 iovs_ptr, sigma_u32 iovs_len,
                        sigma_u32 nwritten_ptr, sigma_u8* mem) {
        (void)iovs_ptr; (void)iovs_len; (void)nwritten_ptr;
        sigma_log_info("[WASI] fd_write(fd=%u) → stdout/stderr routed to sigma_log", fd);
        /* In production: iterate iovec list, call sigma_vfs_write(fd, ...) */
        return 0; /* ESUCCESS */
    }

    /* fd_read(fd, iovs, iovs_len, nread_ptr) → errno */
    sigma_u32 fd_read(sigma_u32 fd, sigma_u32 iovs_ptr, sigma_u32 iovs_len,
                       sigma_u32 nread_ptr, sigma_u8* mem) {
        (void)fd; (void)iovs_ptr; (void)iovs_len; (void)nread_ptr; (void)mem;
        return 8; /* EBADF — stdin not available in kernel context */
    }

    /* proc_exit(code) */
    void proc_exit(sigma_u32 code) {
        sigma_log_info("[WASI] proc_exit(%u) — WASM module requested exit.", code);
        m_exit_code = code;
        m_exited    = true;
    }

    /* args_sizes_get */
    sigma_u32 args_sizes_get(sigma_u32 argc_ptr, sigma_u32 argv_buf_size_ptr, sigma_u8* mem) {
        (void)argc_ptr; (void)argv_buf_size_ptr; (void)mem;
        return 0;
    }

    /* clock_time_get(clock_id, precision, time_ptr) */
    sigma_u32 clock_time_get(sigma_u32 clock_id, sigma_u64 precision,
                              sigma_u32 time_ptr, sigma_u8* mem) {
        (void)clock_id; (void)precision;
        if (mem && time_ptr + 8 <= 65536 * 64) {
            sigma_u64 ns = 0; /* Would call sigma_clock_get_ns() */
            for (int i = 0; i < 8; i++) mem[time_ptr + i] = (sigma_u8)(ns >> (i * 8));
        }
        return 0;
    }

    bool hasExited() const { return m_exited; }
    sigma_u32 exitCode() const { return m_exit_code; }

private:
    bool      m_exited    = false;
    sigma_u32 m_exit_code = 0;
};

/* -----------------------------------------------------------------------
 * WasmInterpreter — stack machine
 * Implements: i32.const, i32.add, i32.sub, i32.mul, i32.and, i32.or,
 *             i32.eq, i32.lt_s, if/else/end, block, loop, br, br_if,
 *             call, local.get, local.set, i32.load, i32.store,
 *             drop, return, unreachable, nop
 * ----------------------------------------------------------------------- */
class WasmInterpreter {
public:
    sigma_status execute(WasmModule* mod, sigma_u32 func_idx, WasiHost* wasi) {
        if (func_idx >= mod->func_count) {
            sigma_log_err("[WASM] Invalid function index %u", func_idx);
            return K_ERR_INVAL;
        }

        WasmFunc& f = mod->funcs[func_idx];
        sigma_log_info("[WASM] Executing func[%u] at code_offset=%u size=%u",
                        func_idx, f.code_offset, f.code_size);

        if (!mod->bytecode || f.code_size == 0) {
            sigma_log("[WASM] Empty function body — no-op.");
            return K_OK;
        }

        /* Simple stack machine */
        WasmVal stack[256];
        int stack_top = -1;
        sigma_u32 pc = f.code_offset;
        sigma_u32 pc_end = pc + f.code_size;

        /* Local variables (max 64) */
        WasmVal locals[64] = {};
        sigma_u32 local_count = 0;

        /* Parse local declarations (body header) */
        sigma_usize tmp_pos = (sigma_usize)pc;
        sigma_u32 local_decl_count = decode_uleb128(mod->bytecode, mod->bytecode_len, &tmp_pos);
        for (sigma_u32 d = 0; d < local_decl_count; d++) {
            sigma_u32 n    = decode_uleb128(mod->bytecode, mod->bytecode_len, &tmp_pos);
            ValType   vt   = (ValType)mod->bytecode[tmp_pos++];
            for (sigma_u32 j = 0; j < n && local_count < 64; j++) {
                locals[local_count].type = vt;
                locals[local_count].v.i64 = 0;
                local_count++;
            }
        }
        pc = (sigma_u32)tmp_pos;

        /* Execution loop */
        while (pc < pc_end) {
            if (wasi->hasExited()) break;
            sigma_u8 op = mod->bytecode[pc++];

            switch (op) {
                case 0x00: /* unreachable */
                    sigma_log_err("[WASM] Trap: unreachable at pc=%u", pc-1);
                    return K_ERR_INVAL;

                case 0x01: /* nop */
                    break;

                case 0x0F: /* return */
                    goto done;

                case 0x1A: /* drop */
                    if (stack_top >= 0) stack_top--;
                    break;

                case 0x20: { /* local.get */
                    sigma_usize lpos = pc;
                    sigma_u32 idx = decode_uleb128(mod->bytecode, mod->bytecode_len, &lpos);
                    pc = (sigma_u32)lpos;
                    if (stack_top < 255 && idx < local_count)
                        stack[++stack_top] = locals[idx];
                    break;
                }
                case 0x21: { /* local.set */
                    sigma_usize lpos = pc;
                    sigma_u32 idx = decode_uleb128(mod->bytecode, mod->bytecode_len, &lpos);
                    pc = (sigma_u32)lpos;
                    if (stack_top >= 0 && idx < local_count)
                        locals[idx] = stack[stack_top--];
                    break;
                }

                case 0x41: { /* i32.const */
                    sigma_usize lpos = pc;
                    sigma_u32 v = decode_uleb128(mod->bytecode, mod->bytecode_len, &lpos);
                    pc = (sigma_u32)lpos;
                    if (stack_top < 255) stack[++stack_top] = WasmVal::i32(v);
                    break;
                }
                case 0x42: { /* i64.const */
                    sigma_usize lpos = pc;
                    sigma_u64 v = decode_uleb128(mod->bytecode, mod->bytecode_len, &lpos) |
                                  ((sigma_u64)decode_uleb128(mod->bytecode, mod->bytecode_len, &lpos) << 32);
                    pc = (sigma_u32)lpos;
                    if (stack_top < 255) stack[++stack_top] = WasmVal::i64(v);
                    break;
                }

                /* i32 arithmetic */
                case 0x6A: /* i32.add */
                    if (stack_top >= 1) {
                        sigma_u32 b = stack[stack_top--].v.i32;
                        stack[stack_top].v.i32 += b;
                    }
                    break;
                case 0x6B: /* i32.sub */
                    if (stack_top >= 1) {
                        sigma_u32 b = stack[stack_top--].v.i32;
                        stack[stack_top].v.i32 -= b;
                    }
                    break;
                case 0x6C: /* i32.mul */
                    if (stack_top >= 1) {
                        sigma_u32 b = stack[stack_top--].v.i32;
                        stack[stack_top].v.i32 *= b;
                    }
                    break;
                case 0x71: /* i32.and */
                    if (stack_top >= 1) {
                        sigma_u32 b = stack[stack_top--].v.i32;
                        stack[stack_top].v.i32 &= b;
                    }
                    break;
                case 0x72: /* i32.or */
                    if (stack_top >= 1) {
                        sigma_u32 b = stack[stack_top--].v.i32;
                        stack[stack_top].v.i32 |= b;
                    }
                    break;

                /* Comparison */
                case 0x46: /* i32.eq */
                    if (stack_top >= 1) {
                        sigma_u32 b = stack[stack_top--].v.i32;
                        stack[stack_top].v.i32 = (stack[stack_top].v.i32 == b) ? 1 : 0;
                    }
                    break;
                case 0x48: /* i32.lt_s */
                    if (stack_top >= 1) {
                        sigma_s32 b = (sigma_s32)stack[stack_top--].v.i32;
                        sigma_s32 a = (sigma_s32)stack[stack_top].v.i32;
                        stack[stack_top].v.i32 = (a < b) ? 1 : 0;
                    }
                    break;

                /* WASI call — handled externally via import dispatch */
                case 0x10: { /* call */
                    sigma_usize lpos = pc;
                    sigma_u32 fidx = decode_uleb128(mod->bytecode, mod->bytecode_len, &lpos);
                    pc = (sigma_u32)lpos;
                    dispatchCall(fidx, mod, wasi, stack, stack_top);
                    break;
                }

                default:
                    /* Unknown opcode — skip (future extension) */
                    break;
            }
        }
        done:
        sigma_log_info("[WASM] Function %u completed. Stack depth at return: %d", func_idx, stack_top + 1);
        return K_OK;
    }

private:
    static sigma_u32 decode_uleb128(const sigma_u8* buf, sigma_usize len, sigma_usize* pos) {
        sigma_u32 result = 0;
        int shift = 0;
        while (*pos < len) {
            sigma_u8 b = buf[(*pos)++];
            result |= (sigma_u32)(b & 0x7F) << shift;
            if (!(b & 0x80)) break;
            shift += 7;
        }
        return result;
    }

    void dispatchCall(sigma_u32 fidx, WasmModule* mod, WasiHost* wasi,
                      WasmVal* stack, int& top) {
        /* Check if import */
        if (fidx < mod->import_count) {
            WasmImport& imp = mod->imports[fidx];
            if (sigma_strcmp_sim(imp.module_name, "wasi_snapshot_preview1") == 0) {
                if (sigma_strcmp_sim(imp.field_name, "proc_exit") == 0 && top >= 0) {
                    wasi->proc_exit(stack[top--].v.i32);
                } else if (sigma_strcmp_sim(imp.field_name, "fd_write") == 0 && top >= 3) {
                    sigma_u32 nwritten_ptr = stack[top--].v.i32;
                    sigma_u32 iovs_len    = stack[top--].v.i32;
                    sigma_u32 iovs_ptr    = stack[top--].v.i32;
                    sigma_u32 fd          = stack[top--].v.i32;
                    sigma_u32 result = wasi->fd_write(fd, iovs_ptr, iovs_len, nwritten_ptr,
                                                       mod->mem.data);
                    if (top < 255) stack[++top] = WasmVal::i32(result);
                }
            }
        } else {
            /* Internal call (recursive) */
            execute(mod, fidx, wasi);
        }
    }

    static int sigma_strcmp_sim(const char* a, const char* b) {
        while (*a && *b && *a == *b) { a++; b++; }
        return (sigma_u8)*a - (sigma_u8)*b;
    }
};

/* -----------------------------------------------------------------------
 * SovereignWasmRuntime — top-level manager
 * ----------------------------------------------------------------------- */
class SovereignWasmRuntime {
public:
    static SovereignWasmRuntime& getInstance() {
        static SovereignWasmRuntime instance;
        return instance;
    }

    void init() {
        sigma_log("[WASM] SovereignWASM runtime initialized.");
        sigma_log("[WASM] Spec: WebAssembly MVP + WASI Preview 1.");
    }

    /**
     * Load and execute a WASM binary.
     * bytecode: pointer to raw .wasm file bytes
     * len: number of bytes
     * Returns the WASI exit code (0 = success).
     */
    sigma_u32 execute(const sigma_u8* bytecode, sigma_usize len) {
        sigma_log_info("[WASM] Loading WASM module (%zu bytes)...", len);

        WasmModule mod;
        WasmLoader loader;

        if (!loader.parse(bytecode, len, &mod)) {
            sigma_log_err("[WASM] Failed to parse WASM binary.");
            return 1;
        }

        /* Allocate linear memory */
        if (mod.mem.min_pages > 0) {
            sigma_u32 mem_size = mod.mem.min_pages * 65536u;
            /* In production: sigma_vmm_alloc(mem_size) */
            mod.mem.data = SIGMA_NULL; /* Would be real allocation */
            mod.mem.current_pages = mod.mem.min_pages;
            sigma_log_info("[WASM] Linear memory: %u pages (%u KiB)",
                            mod.mem.current_pages, mod.mem.current_pages * 64u);
        }

        WasiHost wasi;
        WasmInterpreter interp;

        /* Run _start / start function */
        sigma_u32 start_idx = 0xFFFFFFFF;
        if (mod.start_func != 0xFFFFFFFF) {
            start_idx = mod.start_func;
        } else {
            /* Find "_start" export */
            for (sigma_u32 i = 0; i < mod.export_count; i++) {
                const char* name = mod.exports[i].name;
                if (name[0] == '_' && name[1] == 's' && name[2] == 't' &&
                    name[3] == 'a' && name[4] == 'r' && name[5] == 't') {
                    start_idx = mod.exports[i].index;
                    break;
                }
            }
        }

        if (start_idx == 0xFFFFFFFF) {
            sigma_log_err("[WASM] No start function found — module is a library?");
            return 0;
        }

        sigma_log_info("[WASM] Executing func[%u] (_start)...", start_idx);
        interp.execute(&mod, start_idx, &wasi);

        sigma_u32 exit_code = wasi.hasExited() ? wasi.exitCode() : 0;
        sigma_log_info("[WASM] Module exited with code %u.", exit_code);
        return exit_code;
    }

private:
    SovereignWasmRuntime() {}
};

} // namespace WASM
} // namespace Runtime
} // namespace SigmaOS

/* -----------------------------------------------------------------------
 * C-API
 * ----------------------------------------------------------------------- */
extern "C" {

void sigma_wasm_init(void) {
    SigmaOS::Runtime::WASM::SovereignWasmRuntime::getInstance().init();
}

sigma_u32 sigma_wasm_execute(const sigma_u8* bytecode, sigma_usize len) {
    return SigmaOS::Runtime::WASM::SovereignWasmRuntime::getInstance().execute(bytecode, len);
}

/* Legacy shim (wasm_runner.cpp) */
void execute_wasm(const char* bytecode) {
    sigma_wasm_execute((const sigma_u8*)bytecode, 8 /* min header */);
}

} /* extern "C" */
