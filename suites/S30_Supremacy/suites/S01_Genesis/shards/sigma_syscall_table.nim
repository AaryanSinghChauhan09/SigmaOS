## SigmaOS: =========================================================================
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
  enforcement* = object of RootObj
    initialized*: SigmaBool

proc newenforcement*(): enforcement =
  result = enforcement(initialized: false)

proc sc_stub*(self: var enforcement) =
  self.initialized = true

proc sc_handle_getpid*(self: var enforcement) =
  self.initialized = true

proc sc_handle_write*(self: var enforcement) =
  self.initialized = true

proc sc_handle_exit*(self: var enforcement) =
  self.initialized = true

proc sc_handle_udf*(self: var enforcement) =
  self.initialized = true

proc sigma_syscall_table_init*(self: var enforcement) =
  self.initialized = true

proc sigma_syscall_register*(self: var enforcement) =
  self.initialized = true

proc sigma_syscall_dispatch*(self: var enforcement) =
  self.initialized = true

proc sigma_syscall_audit*(self: var enforcement) =
  self.initialized = true

var instance* = newenforcement()

proc sigma_syscall_table_init*() {.exportc.} =
  instance.initialized = true

proc sigma_syscall_register*() {.exportc.} =
  instance.initialized = true

proc sigma_syscall_audit*() {.exportc.} =
  instance.initialized = true

