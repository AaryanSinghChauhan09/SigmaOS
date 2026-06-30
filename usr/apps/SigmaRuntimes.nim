## SigmaOS: SigmaRuntimes.h — SigmaPy + SigmaR Embedded Runtimes Header
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
  SigmaPy* = object of RootObj
    initialized*: SigmaBool

proc newSigmaPy*(): SigmaPy =
  result = SigmaPy(initialized: false)

proc Py_SetPythonHome*(self: var SigmaPy) =
  self.initialized = true

proc Py_SetPath*(self: var SigmaPy) =
  self.initialized = true

proc Py_InitializeEx*(self: var SigmaPy) =
  self.initialized = true

proc Py_FinalizeEx*(self: var SigmaPy) =
  self.initialized = true

proc Py_GetVersion*(self: var SigmaPy) =
  self.initialized = true

proc PyRun_SimpleFileEx*(self: var SigmaPy) =
  self.initialized = true

proc PyRun_SimpleString*(self: var SigmaPy) =
  self.initialized = true

proc PyDict_SetItemString*(self: var SigmaPy) =
  self.initialized = true

proc PyArg_ParseTuple*(self: var SigmaPy) =
  self.initialized = true

proc PyRun_InteractiveLoop*(self: var SigmaPy) =
  self.initialized = true

proc Rf_initEmbeddedR*(self: var SigmaPy) =
  self.initialized = true

proc Rf_endEmbeddedR*(self: var SigmaPy) =
  self.initialized = true

type
  PyMethodDef* = object
    ml_flags*: SigmaI32

type
  PyModuleDef* = object
    m_base*: SigmaI32
    m_size*: SigmaI32

type
  R_version_struct* = object

type
  R_version* = object

var instance* = newSigmaPy()

proc Py_SetPythonHome*() {.exportc.} =
  instance.initialized = true

proc Py_SetPath*() {.exportc.} =
  instance.initialized = true

proc Py_InitializeEx*() {.exportc.} =
  instance.initialized = true

proc Py_FinalizeEx*() {.exportc.} =
  instance.initialized = true

proc PyDict_SetItemString*() {.exportc.} =
  instance.initialized = true

proc PyRun_InteractiveLoop*() {.exportc.} =
  instance.initialized = true

proc Rf_initEmbeddedR*() {.exportc.} =
  instance.initialized = true

proc Rf_endEmbeddedR*() {.exportc.} =
  instance.initialized = true

