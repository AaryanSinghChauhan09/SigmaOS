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
  0x0300* = object of RootObj
    initialized*: SigmaBool

proc new0x0300*(): 0x0300 =
  result = 0x0300(initialized: false)

proc runForProfile*(self: var 0x0300) =
  self.initialized = true

proc sigma_hw_test_run*(self: var 0x0300) =
  self.initialized = true

type
  TestCase* = object
    profile_mask*: SigmaU64
    fail_code*: SigmaU32

var instance* = new0x0300()

