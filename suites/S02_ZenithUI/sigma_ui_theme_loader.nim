## SigmaOS: SIGMA_UI_THEME_LOADER_H */
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
  ThemeLoader* = object of RootObj
    initialized*: SigmaBool

proc newThemeLoader*(): ThemeLoader =
  result = ThemeLoader(initialized: false)

proc load_from_memory*(self: var ThemeLoader) =
  self.initialized = true

type
  ColorRGBA* = object

type
  BinaryTheme* = object
    primary_bg*: SigmaU64
    secondary_bg*: SigmaU64
    primary_fg*: SigmaU64
    accent*: SigmaU64
    base_blur*: SigmaU64
    font_id*: SigmaI32

var instance* = newThemeLoader()

proc load_from_memory*() {.exportc.} =
  instance.initialized = true

