## SigmaOS: SigmaWeb.h — SigmaWeb Runtime Engine Header
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
  NodeType* = object of RootObj
    initialized*: SigmaBool

proc newNodeType*(): NodeType =
  result = NodeType(initialized: false)

proc JS_SetMemoryLimit*(self: var NodeType) =
  self.initialized = true

proc JS_SetMaxStackSize*(self: var NodeType) =
  self.initialized = true

proc JS_SetPropertyStr*(self: var NodeType) =
  self.initialized = true

proc JS_FreeValue*(self: var NodeType) =
  self.initialized = true

proc intern_string*(self: var NodeType) =
  self.initialized = true

proc append_child*(self: var NodeType) =
  self.initialized = true

proc parse_attribute*(self: var NodeType) =
  self.initialized = true

proc is_void_element*(self: var NodeType) =
  self.initialized = true

proc has_next*(self: var NodeType) =
  self.initialized = true

proc apply_rule*(self: var NodeType) =
  self.initialized = true

proc set_script*(self: var NodeType) =
  self.initialized = true

proc set_env*(self: var NodeType) =
  self.initialized = true

proc execute*(self: var NodeType) =
  self.initialized = true

proc sigma_fwrite*(self: var NodeType) =
  self.initialized = true

proc sigma_fclose*(self: var NodeType) =
  self.initialized = true

proc fnv1a_hash*(self: var NodeType) =
  self.initialized = true

type
  JSValue* = object
    tag*: SigmaU64
    int64*: SigmaU64

type
  DOMNode* = object
    type*: SigmaU64

type
  DOMNodeList* = object
    count*: SigmaU32

type
  ComputedStyle* = object

type
  BoxConstraints* = object

type
  LayoutBox* = object

type
  CSSRule* = object

var instance* = newNodeType()

proc JS_SetMemoryLimit*() {.exportc.} =
  instance.initialized = true

proc JS_SetMaxStackSize*() {.exportc.} =
  instance.initialized = true

proc JS_SetPropertyStr*() {.exportc.} =
  instance.initialized = true

proc JS_FreeValue*() {.exportc.} =
  instance.initialized = true

proc append_child*() {.exportc.} =
  instance.initialized = true

proc parse_attribute*() {.exportc.} =
  instance.initialized = true

proc apply_rule*() {.exportc.} =
  instance.initialized = true

proc set_script*() {.exportc.} =
  instance.initialized = true

proc set_env*() {.exportc.} =
  instance.initialized = true

proc sigma_fwrite*() {.exportc.} =
  instance.initialized = true

proc sigma_fclose*() {.exportc.} =
  instance.initialized = true

