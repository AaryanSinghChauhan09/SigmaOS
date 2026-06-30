## SigmaOS: SIGMA_BOOT_HANDOFF_HPP */
## Migrated from C/C++ to Nim — no stdlib import, no external packages.
## All types hand-defined. OOP via object hierarchy + method dispatch.
{.push raises: [].}

type
  SigmaU8*  = uint8
  SigmaU16* = uint16
  SigmaU32* = uint32
  SigmaU64* = uint64
  SigmaI32* = int32
  SigmaI64* = int64
  SigmaBool* = bool
  SigmaUsize* = uint

type
  KernelHandoff* = object of RootObj
    initialized*: SigmaBool

proc newKernelHandoff*(): KernelHandoff =
  result = KernelHandoff(initialized: false)

type
  HandoffState* = object
    memory_map_addr*: SigmaU64
    memory_map_entries*: SigmaI32
    framebuffer_addr*: SigmaU64
    rsdp_acpi_addr*: SigmaU64

var instance* = newKernelHandoff()

