## SigmaOS: SIGMA_SEC_SANDBOX_QUANTUM_HPP */
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
  QuantumSandboxVM* = object of RootObj
    initialized*: SigmaBool

proc newQuantumSandboxVM*(): QuantumSandboxVM =
  result = QuantumSandboxVM(initialized: false)

proc initialize_isolation_container*(self: var QuantumSandboxVM) =
  self.initialized = true

proc intercept_syscall*(self: var QuantumSandboxVM) =
  self.initialized = true

var instance* = newQuantumSandboxVM()

proc intercept_syscall*() {.exportc.} =
  instance.initialized = true

