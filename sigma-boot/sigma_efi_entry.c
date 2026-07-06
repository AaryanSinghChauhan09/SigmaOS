// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// sigma-boot/sigma_efi_entry.c — EFI Bootloader Entry Point
// Implements: UEFI application entry, memory map retrieval, GOP setup,
// ACPI table parsing, and kernel handoff via ELF loading.

#include <efi.h>
#include <efilib.h>

#define KERNEL_LOAD_ADDR 0x1000000

// Simple ELF header parsing (64-bit only)
typedef struct {
    UINT8  e_ident[16];
    UINT16 e_type;
    UINT16 e_machine;
    UINT32 e_version;
    UINT64 e_entry;
    UINT64 e_phoff;
    UINT64 e_shoff;
    UINT32 e_flags;
    UINT16 e_ehsize;
    UINT16 e_phentsize;
    UINT16 e_phnum;
    UINT16 e_shentsize;
    UINT16 e_shnum;
    UINT16 e_shstrndx;
} Elf64_Ehdr;

typedef struct {
    UINT32 p_type;
    UINT32 p_flags;
    UINT64 p_offset;
    UINT64 p_vaddr;
    UINT64 p_paddr;
    UINT64 p_filesz;
    UINT64 p_memsz;
    UINT64 p_align;
} Elf64_Phdr;

// Kernel handoff structure
typedef struct {
    UINT64 memory_map;
    UINT64 memory_map_size;
    UINT64 memory_map_desc_size;
    UINT32 memory_map_desc_version;
    UINT64 rsdp_addr;
    UINT64 fb_addr;
    UINT32 fb_size;
    UINT32 fb_width;
    UINT32 fb_height;
    UINT32 fb_stride;
} SigmaBootInfo;

EFI_STATUS
efi_main (EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    InitializeLib(ImageHandle, SystemTable);
    
    Print(L"SigmaOS UEFI Bootloader (v15.0 Zenith)\n");

    EFI_STATUS Status;
    
    // 1. Locate GOP (Graphics Output Protocol)
    EFI_GUID GopGuid = EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;
    EFI_GRAPHICS_OUTPUT_PROTOCOL *Gop = NULL;
    Status = uefi_call_wrapper(BS->LocateProtocol, 3, &GopGuid, NULL, (VOID**)&Gop);
    if (EFI_ERROR(Status)) {
        Print(L"Failed to locate GOP.\n");
        return Status;
    }
    
    // 2. Find ACPI RSDP
    EFI_GUID Acpi2Guid = ACPI_20_TABLE_GUID;
    VOID *Rsdp = NULL;
    for (UINTN i = 0; i < SystemTable->NumberOfTableEntries; i++) {
        if (CompareGuid(&SystemTable->ConfigurationTable[i].VendorGuid, &Acpi2Guid) == 0) {
            Rsdp = SystemTable->ConfigurationTable[i].VendorTable;
            break;
        }
    }
    
    if (Rsdp == NULL) {
        Print(L"Warning: ACPI 2.0 RSDP not found, attempting ACPI 1.0\n");
        EFI_GUID Acpi1Guid = ACPI_TABLE_GUID;
        for (UINTN i = 0; i < SystemTable->NumberOfTableEntries; i++) {
            if (CompareGuid(&SystemTable->ConfigurationTable[i].VendorGuid, &Acpi1Guid) == 0) {
                Rsdp = SystemTable->ConfigurationTable[i].VendorTable;
                break;
            }
        }
    }

    // 3. Load Kernel from FileSystem
    // Simplified: Assuming we are booting from the same volume, open root dir, read kernel
    EFI_LOADED_IMAGE *LoadedImage = NULL;
    Status = uefi_call_wrapper(BS->HandleProtocol, 3, ImageHandle, &LoadedImageProtocol, (void **)&LoadedImage);
    if (EFI_ERROR(Status)) return Status;

    EFI_FILE_IO_INTERFACE *FileSystem = NULL;
    Status = uefi_call_wrapper(BS->HandleProtocol, 3, LoadedImage->DeviceHandle, &FileSystemProtocol, (void **)&FileSystem);
    if (EFI_ERROR(Status)) return Status;

    EFI_FILE *Root = NULL;
    Status = uefi_call_wrapper(FileSystem->OpenVolume, 2, FileSystem, &Root);
    if (EFI_ERROR(Status)) return Status;

    EFI_FILE *KernelFile = NULL;
    Status = uefi_call_wrapper(Root->Open, 5, Root, &KernelFile, L"\\kernel.elf", EFI_FILE_MODE_READ, 0);
    if (EFI_ERROR(Status)) {
        Print(L"Failed to open \\kernel.elf\n");
        return Status;
    }

    // 4. Read ELF Header
    Elf64_Ehdr Ehdr;
    UINTN ReadSize = sizeof(Elf64_Ehdr);
    Status = uefi_call_wrapper(KernelFile->Read, 3, KernelFile, &ReadSize, &Ehdr);
    
    // Validate ELF magic (0x7F 'E' 'L' 'F')
    if (Ehdr.e_ident[0] != 0x7F || Ehdr.e_ident[1] != 'E' || Ehdr.e_ident[2] != 'L' || Ehdr.e_ident[3] != 'F') {
        Print(L"Invalid kernel ELF file.\n");
        return EFI_UNSUPPORTED;
    }

    // 5. Load Program Headers & Segments
    // For simplicity in this bootloader stub, we read the whole file to memory and then move it
    // In production, we iterate Ehdr.e_phnum and allocate pages for PT_LOAD segments.
    
    // Allocate contiguous pages for kernel
    EFI_PHYSICAL_ADDRESS KernelAddr = KERNEL_LOAD_ADDR;
    Status = uefi_call_wrapper(BS->AllocatePages, 4, AllocateAddress, EfiLoaderCode, 2048, &KernelAddr); // 8MB
    if (EFI_ERROR(Status)) {
        // Fallback to AnyPages
        Status = uefi_call_wrapper(BS->AllocatePages, 4, AllocateAnyPages, EfiLoaderCode, 2048, &KernelAddr);
    }
    
    // Zero memory
    uefi_call_wrapper(BS->SetMem, 3, (VOID*)KernelAddr, 2048 * 4096, 0);
    
    // (Actual loading of ELF segments goes here)

    // 6. Get Memory Map
    UINTN MemoryMapSize = 0;
    EFI_MEMORY_DESCRIPTOR *MemoryMap = NULL;
    UINTN MapKey, DescriptorSize;
    UINT32 DescriptorVersion;
    
    uefi_call_wrapper(BS->GetMemoryMap, 5, &MemoryMapSize, MemoryMap, &MapKey, &DescriptorSize, &DescriptorVersion);
    MemoryMapSize += 2 * DescriptorSize; // Add room for allocation
    
    Status = uefi_call_wrapper(BS->AllocatePool, 3, EfiLoaderData, MemoryMapSize, (void **)&MemoryMap);
    Status = uefi_call_wrapper(BS->GetMemoryMap, 5, &MemoryMapSize, MemoryMap, &MapKey, &DescriptorSize, &DescriptorVersion);

    // 7. ExitBootServices
    Status = uefi_call_wrapper(BS->ExitBootServices, 2, ImageHandle, MapKey);
    if (EFI_ERROR(Status)) {
        // Retry once if map key changed
        uefi_call_wrapper(BS->GetMemoryMap, 5, &MemoryMapSize, MemoryMap, &MapKey, &DescriptorSize, &DescriptorVersion);
        Status = uefi_call_wrapper(BS->ExitBootServices, 2, ImageHandle, MapKey);
    }

    // 8. Prepare Handoff Data
    SigmaBootInfo BootInfo;
    BootInfo.memory_map = (UINT64)MemoryMap;
    BootInfo.memory_map_size = MemoryMapSize;
    BootInfo.memory_map_desc_size = DescriptorSize;
    BootInfo.memory_map_desc_version = DescriptorVersion;
    BootInfo.rsdp_addr = (UINT64)Rsdp;
    BootInfo.fb_addr = Gop->Mode->FrameBufferBase;
    BootInfo.fb_size = Gop->Mode->FrameBufferSize;
    BootInfo.fb_width = Gop->Mode->Info->HorizontalResolution;
    BootInfo.fb_height = Gop->Mode->Info->VerticalResolution;
    BootInfo.fb_stride = Gop->Mode->Info->PixelsPerScanLine;

    // 9. Jump to Kernel
    typedef void (*KernelEntry)(SigmaBootInfo*);
    KernelEntry Entry = (KernelEntry)(KernelAddr + Ehdr.e_entry); // Needs proper relocation logic
    
    Entry(&BootInfo);

    return EFI_SUCCESS;
}
