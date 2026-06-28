/**
 * =========================================================================
 * Σ SIGMAOS: PE32+ TYPE DEFINITIONS
 * =========================================================================
 * Windows Portable Executable (PE32+) structures for sigma-wine.
 * Mirrors Microsoft's winnt.h PE layout without depending on it.
 * =========================================================================
 */
#pragma once
#include "../sigma_kernel_types.h"

/* ---- Basic Win32 types ---- */
typedef sigma_u8   BYTE;
typedef sigma_u16  WORD;
typedef sigma_u32  DWORD;
typedef sigma_u64  ULONGLONG;
typedef sigma_u32  ULONG;
typedef sigma_u16  USHORT;
typedef sigma_u8   UCHAR;
typedef sigma_s32  LONG;
typedef void*      PVOID;
typedef void*      LPVOID;
typedef char*      LPSTR;
typedef const char* LPCSTR;

/* ---- MZ / DOS header ---- */
#define IMAGE_DOS_SIGNATURE  0x5A4D   /* "MZ" */
#define IMAGE_NT_SIGNATURE   0x00004550 /* "PE\0\0" */

typedef struct {
    WORD  e_magic;      /* MZ magic */
    WORD  e_cblp;
    WORD  e_cp;
    WORD  e_crlc;
    WORD  e_cparhdr;
    WORD  e_minalloc;
    WORD  e_maxalloc;
    WORD  e_ss;
    WORD  e_sp;
    WORD  e_csum;
    WORD  e_ip;
    WORD  e_cs;
    WORD  e_lfarlc;
    WORD  e_ovno;
    WORD  e_res[4];
    WORD  e_oemid;
    WORD  e_oeminfo;
    WORD  e_res2[10];
    LONG  e_lfanew;     /* Offset to PE header */
} IMAGE_DOS_HEADER;

/* ---- PE file header ---- */
#define IMAGE_FILE_MACHINE_AMD64  0x8664
#define IMAGE_FILE_MACHINE_I386   0x014C
#define IMAGE_FILE_MACHINE_ARM64  0xAA64

typedef struct {
    WORD  Machine;
    WORD  NumberOfSections;
    DWORD TimeDateStamp;
    DWORD PointerToSymbolTable;
    DWORD NumberOfSymbols;
    WORD  SizeOfOptionalHeader;
    WORD  Characteristics;
} IMAGE_FILE_HEADER;

/* Characteristics flags */
#define IMAGE_FILE_EXECUTABLE_IMAGE  0x0002
#define IMAGE_FILE_DLL               0x2000
#define IMAGE_FILE_LARGE_ADDRESS_AWARE 0x0020

/* ---- Optional header (PE32+) ---- */
#define IMAGE_NT_OPTIONAL_HDR64_MAGIC 0x020B

typedef struct {
    WORD        Magic;              /* 0x020B for PE32+ */
    BYTE        MajorLinkerVersion;
    BYTE        MinorLinkerVersion;
    DWORD       SizeOfCode;
    DWORD       SizeOfInitializedData;
    DWORD       SizeOfUninitializedData;
    DWORD       AddressOfEntryPoint; /* RVA of entry point */
    DWORD       BaseOfCode;
    ULONGLONG   ImageBase;           /* Preferred load address */
    DWORD       SectionAlignment;
    DWORD       FileAlignment;
    WORD        MajorOSVersion;
    WORD        MinorOSVersion;
    WORD        MajorImageVersion;
    WORD        MinorImageVersion;
    WORD        MajorSubsystemVersion;
    WORD        MinorSubsystemVersion;
    DWORD       Win32VersionValue;
    DWORD       SizeOfImage;
    DWORD       SizeOfHeaders;
    DWORD       CheckSum;
    WORD        Subsystem;
    WORD        DllCharacteristics;
    ULONGLONG   SizeOfStackReserve;
    ULONGLONG   SizeOfStackCommit;
    ULONGLONG   SizeOfHeapReserve;
    ULONGLONG   SizeOfHeapCommit;
    DWORD       LoaderFlags;
    DWORD       NumberOfRvaAndSizes;
} IMAGE_OPTIONAL_HEADER64;

/* Subsystem values */
#define IMAGE_SUBSYSTEM_CONSOLE    3
#define IMAGE_SUBSYSTEM_WINDOWS_GUI 2

/* ---- Data directory ---- */
#define IMAGE_NUMBEROF_DIRECTORY_ENTRIES 16
#define IMAGE_DIRECTORY_ENTRY_EXPORT     0
#define IMAGE_DIRECTORY_ENTRY_IMPORT     1
#define IMAGE_DIRECTORY_ENTRY_RESOURCE   2
#define IMAGE_DIRECTORY_ENTRY_EXCEPTION  3
#define IMAGE_DIRECTORY_ENTRY_BASERELOC  5
#define IMAGE_DIRECTORY_ENTRY_TLS        9
#define IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG 10

typedef struct {
    DWORD VirtualAddress;
    DWORD Size;
} IMAGE_DATA_DIRECTORY;

/* ---- NT headers (PE32+) ---- */
typedef struct {
    DWORD                   Signature;
    IMAGE_FILE_HEADER       FileHeader;
    IMAGE_OPTIONAL_HEADER64 OptionalHeader;
    IMAGE_DATA_DIRECTORY    DataDirectory[IMAGE_NUMBEROF_DIRECTORY_ENTRIES];
} IMAGE_NT_HEADERS64;

/* ---- Section header ---- */
#define IMAGE_SIZEOF_SHORT_NAME 8
#define IMAGE_SCN_CNT_CODE      0x00000020
#define IMAGE_SCN_CNT_IDATA     0x00000040
#define IMAGE_SCN_CNT_UDATA     0x00000080
#define IMAGE_SCN_MEM_EXECUTE   0x20000000
#define IMAGE_SCN_MEM_READ      0x40000000
#define IMAGE_SCN_MEM_WRITE     0x80000000

typedef struct {
    BYTE  Name[IMAGE_SIZEOF_SHORT_NAME];
    DWORD VirtualSize;
    DWORD VirtualAddress;
    DWORD SizeOfRawData;
    DWORD PointerToRawData;
    DWORD PointerToRelocations;
    DWORD PointerToLinenumbers;
    WORD  NumberOfRelocations;
    WORD  NumberOfLinenumbers;
    DWORD Characteristics;
} IMAGE_SECTION_HEADER;

/* ---- Import table ---- */
typedef struct {
    DWORD OriginalFirstThunk; /* RVA to INT */
    DWORD TimeDateStamp;
    DWORD ForwarderChain;
    DWORD Name;               /* RVA to DLL name */
    DWORD FirstThunk;         /* RVA to IAT */
} IMAGE_IMPORT_DESCRIPTOR;

typedef struct {
    union {
        ULONGLONG ForwarderString;
        ULONGLONG Function;
        ULONGLONG Ordinal;
        ULONGLONG AddressOfData; /* RVA to IMAGE_IMPORT_BY_NAME */
    } u1;
} IMAGE_THUNK_DATA64;

typedef struct {
    WORD  Hint;
    BYTE  Name[1]; /* Variable-length */
} IMAGE_IMPORT_BY_NAME;

/* ---- Export table ---- */
typedef struct {
    DWORD Characteristics;
    DWORD TimeDateStamp;
    WORD  MajorVersion;
    WORD  MinorVersion;
    DWORD Name;                /* RVA to DLL name */
    DWORD Base;
    DWORD NumberOfFunctions;
    DWORD NumberOfNames;
    DWORD AddressOfFunctions;  /* RVA to EAT */
    DWORD AddressOfNames;      /* RVA to name table */
    DWORD AddressOfNameOrdinals;
} IMAGE_EXPORT_DIRECTORY;

/* ---- Base relocation ---- */
#define IMAGE_REL_BASED_DIR64 10

typedef struct {
    DWORD VirtualAddress;
    DWORD SizeOfBlock;
    /* WORD TypeOffset[]; follows */
} IMAGE_BASE_RELOCATION;

/* ---- TLS directory ---- */
typedef struct {
    ULONGLONG StartAddressOfRawData;
    ULONGLONG EndAddressOfRawData;
    ULONGLONG AddressOfIndex;
    ULONGLONG AddressOfCallBacks; /* Array of TLS callbacks, NULL-terminated */
    DWORD     SizeOfZeroFill;
    DWORD     Characteristics;
} IMAGE_TLS_DIRECTORY64;

/* Convenience macro: RVA → pointer given image base */
#define PE_RVA_TO_PTR(base, rva) \
    ((void*)((sigma_u8*)(base) + (sigma_u32)(rva)))
