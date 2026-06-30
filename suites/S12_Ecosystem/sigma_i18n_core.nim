## SigmaOS: SIGMA_I18N_CORE_HPP */
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
  Locale* = object of RootObj
    initialized*: SigmaBool

proc newLocale*(): Locale =
  result = Locale(initialized: false)

proc set_locale*(self: var Locale) =
  self.initialized = true

proc translate*(self: var Locale) =
  self.initialized = true

var instance* = newLocale()

proc set_locale*() {.exportc.} =
  instance.initialized = true

