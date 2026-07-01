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
  SovereignAudioCapture* = object
    head*: SigmaU32
    tail*: SigmaU32
    capturing*: SigmaU64

type
  SovereignTranscriptionEngine* = object
    frames_processed*: SigmaU64

type
  SovereignHIDBridge* = object
    chars_injected*: SigmaU64
    linux_evdev_mode*: SigmaU64

type
  SovereignVoiceShard* = object
    audio*: SigmaU64
    engine*: SigmaU64
    hid*: SigmaU64
    wake_active*: SigmaU64
    events_processed*: SigmaU64

proc audio_init*() {.exportc.} =
  discard

proc audio_start_capture*() {.exportc.} =
  discard

proc audio_stop_capture*() {.exportc.} =
  discard

proc transcribe_init*() {.exportc.} =
  discard

proc postprocess_text*() {.exportc.} =
  discard

proc transcribe_run*() {.exportc.} =
  discard

proc hid_init*() {.exportc.} =
  discard

proc hid_inject*() {.exportc.} =
  discard

proc voice_init*() {.exportc.} =
  discard

proc voice_activate_wake_key*() {.exportc.} =
  discard

proc voice_process_event*() {.exportc.} =
  discard

proc voice_audit*() {.exportc.} =
  discard

