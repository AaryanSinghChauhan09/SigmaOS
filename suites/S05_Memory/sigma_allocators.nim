## SigmaOS: sigma_allocators module
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
  BuddyAllocator* = object of RootObj
    initialized*: SigmaBool

proc newBuddyAllocator*(): BuddyAllocator =
  result = BuddyAllocator(initialized: false)

proc deallocate*(self: var BuddyAllocator) =
  self.initialized = true

proc start_memory_lattice*(self: var BuddyAllocator) =
  self.initialized = true

var instance* = newBuddyAllocator()

proc deallocate*() {.exportc.} =
  instance.initialized = true

proc start_memory_lattice*() {.exportc.} =
  instance.initialized = true

