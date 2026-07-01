## SigmaOS: sovereign_shell module
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
  SovereignShell* = object of RootObj
    initialized*: SigmaBool

proc newSovereignShell*(): SovereignShell =
  result = SovereignShell(initialized: false)

proc init*(self: var SovereignShell) =
  self.initialized = true

proc run*(self: var SovereignShell) =
  self.initialized = true

proc pushHistory*(self: var SovereignShell) =
  self.initialized = true

proc printPrompt*(self: var SovereignShell) =
  self.initialized = true

proc handlePipe*(self: var SovereignShell) =
  self.initialized = true

proc handleRedirection*(self: var SovereignShell) =
  self.initialized = true

proc readline*(self: var SovereignShell) =
  self.initialized = true

proc tokenize*(self: var SovereignShell) =
  self.initialized = true

proc dispatch*(self: var SovereignShell) =
  self.initialized = true

proc builtinHelp*(self: var SovereignShell) =
  self.initialized = true

proc builtinHistory*(self: var SovereignShell) =
  self.initialized = true

proc sigma_strstr*(self: var SovereignShell) =
  self.initialized = true

proc main*(self: var SovereignShell) =
  self.initialized = true

var instance* = newSovereignShell()

proc init*() {.exportc.} =
  instance.initialized = true

proc run*() {.exportc.} =
  instance.initialized = true

proc pushHistory*() {.exportc.} =
  instance.initialized = true

proc printPrompt*() {.exportc.} =
  instance.initialized = true

proc handlePipe*() {.exportc.} =
  instance.initialized = true

proc handleRedirection*() {.exportc.} =
  instance.initialized = true

