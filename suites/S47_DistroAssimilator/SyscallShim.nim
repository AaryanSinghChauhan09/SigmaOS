## SigmaOS: SyscallShim module
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
  SyscallShim* = object of RootObj
    initialized*: SigmaBool

proc newSyscallShim*(): SyscallShim =
  result = SyscallShim(initialized: false)

proc handle_linux_syscall*(self: var SyscallShim) =
  self.initialized = true

var instance* = newSyscallShim()

proc handle_linux_syscall*() {.exportc.} =
  instance.initialized = true

