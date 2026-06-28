/**
 * =========================================================================
 * Σ SIGMAOS: PE32+ LOADER (sigma-pe)  — Stage 1
 * =========================================================================
 * Parses Windows PE32+ executables and maps them into a SigmaOS address
 * space. Parallel to sigma_linux_compat.cpp which handles ELF64.
 *
 * Status: skeleton — VMM integration pending Phase 0 kernel completion.
 * =========================================================================
 */
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/compat/sigma_pe_types.h"
#include "../../../include/compat/sigma_nt_types.h"

namespace SigmaOS { namespace Compat { namespace Win32 {

/* -----------------------------------------------------------------------
 * Loaded PE image descriptor
 * ----------------------------------------------------------------------- */
struct PeLoadedImage {
    sigma_u64   image_base;       /* actual load address (after relocation) */
    sigma_u64   preferred_base;   /* IMAGE_OPTIONAL_HEADER64.ImageBase */
    sigma_u64   entry_point;      /* absolute VA of entry point */
    sigma_u64   size_of_image;
    bool        is_dll;
    bool        is_pie;
    char        subsystem[16];    /* "Console" or "GUI" */
    /* Section map — up to 64 sections */
    struct Section {
        char     name[9];
        sigma_u64 va;
        sigma_u64 size;
        sigma_u32 perms;          /* R=4 W=2 X=1 */
        sigma_u8* mem;            /* NULL until VMM integration */
    } sections[64];
    sigma_u32   section_count;
    /* Import table */
    struct Import {
        char dll_name[64];
        /* thunks resolved after DLL load */
    } imports[128];
    sigma_u32   import_count;
};

/* -----------------------------------------------------------------------
 * PeLoader — main class
 * ----------------------------------------------------------------------- */
class PeLoader {
public:
    /**
     * load — parse a PE32+ image from memory.
     * In production: buf points to file data read from sigma-vfs.
     * @param buf    raw file bytes
     * @param len    file size
     * @param out    output descriptor (caller-allocated)
     * @return true on success
     */
    bool load(const sigma_u8* buf, sigma_usize len, PeLoadedImage* out) {
        if (len < sizeof(IMAGE_DOS_HEADER)) {
            sigma_log_err("[PE] File too small (%zu bytes)", len);
            return false;
        }

        const IMAGE_DOS_HEADER* dos =
            reinterpret_cast<const IMAGE_DOS_HEADER*>(buf);

        if (dos->e_magic != IMAGE_DOS_SIGNATURE) {
            sigma_log_err("[PE] Invalid MZ magic: 0x%04X", dos->e_magic);
            return false;
        }

        if ((sigma_usize)dos->e_lfanew + sizeof(IMAGE_NT_HEADERS64) > len) {
            sigma_log_err("[PE] e_lfanew (0x%X) out of range", dos->e_lfanew);
            return false;
        }

        const IMAGE_NT_HEADERS64* nt =
            reinterpret_cast<const IMAGE_NT_HEADERS64*>(buf + dos->e_lfanew);

        if (nt->Signature != IMAGE_NT_SIGNATURE) {
            sigma_log_err("[PE] Invalid PE signature: 0x%08X", nt->Signature);
            return false;
        }
        if (nt->FileHeader.Machine != IMAGE_FILE_MACHINE_AMD64) {
            sigma_log_err("[PE] Unsupported machine: 0x%04X (only AMD64 supported)",
                          nt->FileHeader.Machine);
            return false;
        }
        if (nt->OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR64_MAGIC) {
            sigma_log_err("[PE] Not a PE32+ optional header");
            return false;
        }

        const IMAGE_OPTIONAL_HEADER64& opt = nt->OptionalHeader;
        out->preferred_base = opt.ImageBase;
        out->is_dll         = (nt->FileHeader.Characteristics & IMAGE_FILE_DLL) != 0;
        out->size_of_image  = opt.SizeOfImage;
        out->section_count  = 0;
        out->import_count   = 0;

        /* Determine load address — ASLR handled by sigma-vmm in production.
         * For now, use preferred base (will be overridden by vmm_alloc). */
        out->image_base  = opt.ImageBase;
        out->entry_point = opt.ImageBase + opt.AddressOfEntryPoint;
        out->is_pie      = false; /* PE32+ always has preferred base */

        /* Subsystem string */
        if (opt.Subsystem == IMAGE_SUBSYSTEM_CONSOLE)
            __builtin_memcpy(out->subsystem, "Console", 8);
        else if (opt.Subsystem == IMAGE_SUBSYSTEM_WINDOWS_GUI)
            __builtin_memcpy(out->subsystem, "GUI", 4);
        else
            __builtin_memcpy(out->subsystem, "Unknown", 8);

        sigma_log_info("[PE] %s machine=AMD64 base=0x%llX entry=0x%llX sections=%u sub=%s",
                       out->is_dll ? "DLL" : "EXE",
                       (unsigned long long)opt.ImageBase,
                       (unsigned long long)out->entry_point,
                       (unsigned)nt->FileHeader.NumberOfSections,
                       out->subsystem);

        /* Map sections */
        const sigma_u8* sec_base =
            reinterpret_cast<const sigma_u8*>(nt) +
            sizeof(DWORD) +                       /* Signature */
            sizeof(IMAGE_FILE_HEADER) +
            nt->FileHeader.SizeOfOptionalHeader;

        sigma_u32 nsec = nt->FileHeader.NumberOfSections;
        if (nsec > 64) nsec = 64;

        for (sigma_u32 i = 0; i < nsec; i++) {
            const IMAGE_SECTION_HEADER* sh =
                reinterpret_cast<const IMAGE_SECTION_HEADER*>(
                    sec_base + i * sizeof(IMAGE_SECTION_HEADER));

            PeLoadedImage::Section& s = out->sections[out->section_count++];
            /* Copy name (8 bytes, may not be NUL-terminated) */
            for (int j = 0; j < 8; j++) s.name[j] = (char)sh->Name[j];
            s.name[8] = '\0';
            s.va   = opt.ImageBase + sh->VirtualAddress;
            s.size = sh->VirtualSize ? sh->VirtualSize : sh->SizeOfRawData;
            s.perms = 0;
            if (sh->Characteristics & IMAGE_SCN_MEM_READ)    s.perms |= 4;
            if (sh->Characteristics & IMAGE_SCN_MEM_WRITE)   s.perms |= 2;
            if (sh->Characteristics & IMAGE_SCN_MEM_EXECUTE) s.perms |= 1;
            s.mem = SIGMA_NULL; /* TODO: sigma_vmm_map_region(s.va, s.size, s.perms) */

            sigma_log_info("[PE]   section %-8s va=0x%llX size=0x%llX %c%c%c",
                           s.name, (unsigned long long)s.va, (unsigned long long)s.size,
                           (s.perms & 4) ? 'R' : '-',
                           (s.perms & 2) ? 'W' : '-',
                           (s.perms & 1) ? 'X' : '-');
        }

        /* Parse import table */
        parseImports(buf, len, nt, out);

        /* Apply base relocations (if load_addr != preferred_base) */
        /* TODO: applyRelocs(buf, len, nt, out, delta); */

        sigma_log("[PE] Load complete. %u sections, %u imports.",
                  out->section_count, out->import_count);
        return true;
    }

    /**
     * inspect — non-executing header dump for `sigma-wine --info`.
     */
    void inspect(const sigma_u8* buf, sigma_usize len) {
        PeLoadedImage img;
        if (load(buf, len, &img)) {
            sigma_log("[PE][INSPECT] ImageBase=0x%llX EntryPoint=0x%llX Type=%s",
                      (unsigned long long)img.image_base,
                      (unsigned long long)img.entry_point,
                      img.is_dll ? "DLL" : "EXE");
        }
    }

private:
    void parseImports(const sigma_u8* buf, sigma_usize len,
                      const IMAGE_NT_HEADERS64* nt, PeLoadedImage* out) {
        const IMAGE_DATA_DIRECTORY& imp_dir =
            nt->DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT];
        if (imp_dir.VirtualAddress == 0 || imp_dir.Size == 0) return;

        /* Convert RVA → file offset via section table */
        sigma_u32 imp_off = rvaToOffset(buf, nt, imp_dir.VirtualAddress);
        if (imp_off == 0) return;

        const IMAGE_IMPORT_DESCRIPTOR* desc =
            reinterpret_cast<const IMAGE_IMPORT_DESCRIPTOR*>(buf + imp_off);

        while (desc->Name != 0 && out->import_count < 128) {
            sigma_u32 name_off = rvaToOffset(buf, nt, desc->Name);
            if (name_off == 0 || name_off >= len) break;

            const char* dll_name = reinterpret_cast<const char*>(buf + name_off);
            PeLoadedImage::Import& imp = out->imports[out->import_count++];
            sigma_usize nlen = 0;
            while (nlen < 63 && dll_name[nlen]) {
                imp.dll_name[nlen] = dll_name[nlen];
                nlen++;
            }
            imp.dll_name[nlen] = '\0';
            sigma_log_info("[PE]   import DLL: %s", imp.dll_name);
            desc++;
        }
    }

    /* Translate RVA → file byte offset using section table */
    sigma_u32 rvaToOffset(const sigma_u8* buf,
                           const IMAGE_NT_HEADERS64* nt, sigma_u32 rva) {
        const sigma_u8* sec_base =
            reinterpret_cast<const sigma_u8*>(nt) +
            sizeof(DWORD) + sizeof(IMAGE_FILE_HEADER) +
            nt->FileHeader.SizeOfOptionalHeader;
        sigma_u32 nsec = nt->FileHeader.NumberOfSections;
        for (sigma_u32 i = 0; i < nsec; i++) {
            const IMAGE_SECTION_HEADER* sh =
                reinterpret_cast<const IMAGE_SECTION_HEADER*>(
                    sec_base + i * sizeof(IMAGE_SECTION_HEADER));
            if (rva >= sh->VirtualAddress &&
                rva <  sh->VirtualAddress + sh->SizeOfRawData) {
                return sh->PointerToRawData + (rva - sh->VirtualAddress);
            }
        }
        return 0;
    }
};

} /* Win32 */ } /* Compat */ } /* SigmaOS */

/* -----------------------------------------------------------------------
 * C API
 * ----------------------------------------------------------------------- */
extern "C" {

sigma_status sigma_pe_load(const sigma_u8* buf, sigma_usize len, void* out_img) {
    SigmaOS::Compat::Win32::PeLoader loader;
    if (!loader.load(buf, len,
            static_cast<SigmaOS::Compat::Win32::PeLoadedImage*>(out_img)))
        return K_ERR_INVAL;
    return K_OK;
}

void sigma_pe_inspect(const sigma_u8* buf, sigma_usize len) {
    SigmaOS::Compat::Win32::PeLoader loader;
    loader.inspect(buf, len);
}

} /* extern "C" */
