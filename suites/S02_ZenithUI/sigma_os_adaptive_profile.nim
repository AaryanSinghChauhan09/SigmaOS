## SigmaOS: SIGMA_OS_ADAPTIVE_PROFILE_HPP */
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
  ProfileManager* = object of RootObj
    initialized*: SigmaBool

proc newProfileManager*(): ProfileManager =
  result = ProfileManager(initialized: false)

proc load_profile*(self: var ProfileManager) =
  self.initialized = true

proc apply_profile_sovereignty*(self: var ProfileManager) =
  self.initialized = true

type
  PortableProfile* = object
    preferred_ui_mode*: SigmaU64
    default_capability_mask*: SigmaI32
    auto_backup_enabled*: SigmaBool
    telemetry_opt_out_level*: SigmaI32

var instance* = newProfileManager()

proc load_profile*() {.exportc.} =
  instance.initialized = true

proc apply_profile_sovereignty*() {.exportc.} =
  instance.initialized = true

