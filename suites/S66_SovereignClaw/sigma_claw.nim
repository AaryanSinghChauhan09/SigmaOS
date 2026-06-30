## SigmaOS: --- C Linkage Bridging --- */
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
  SovereignClaw* = object of RootObj
    initialized*: SigmaBool

proc newSovereignClaw*(): SovereignClaw =
  result = SovereignClaw(initialized: false)

proc initialize*(self: var SovereignClaw) =
  self.initialized = true

proc set_sandbox_mode*(self: var SovereignClaw) =
  self.initialized = true

proc process_intent*(self: var SovereignClaw) =
  self.initialized = true

proc execute_skill*(self: var SovereignClaw) =
  self.initialized = true

proc persist_context*(self: var SovereignClaw) =
  self.initialized = true

proc handle_message*(self: var SovereignClaw) =
  self.initialized = true

proc render_canvas*(self: var SovereignClaw) =
  self.initialized = true

proc claw_gateway_init*(self: var SovereignClaw) =
  self.initialized = true

proc claw_route_message*(self: var SovereignClaw) =
  self.initialized = true

proc claw_render_canvas*(self: var SovereignClaw) =
  self.initialized = true

proc claw_execute_tool*(self: var SovereignClaw) =
  self.initialized = true

proc claw_sandbox_policy*(self: var SovereignClaw) =
  self.initialized = true

proc claw_daemon_init*(self: var SovereignClaw) =
  self.initialized = true

type
  ClawIntent* = object
    priority*: SigmaU32
    requires_sandboxing*: SigmaBool

var instance* = newSovereignClaw()

proc initialize*() {.exportc.} =
  instance.initialized = true

proc set_sandbox_mode*() {.exportc.} =
  instance.initialized = true

proc process_intent*() {.exportc.} =
  instance.initialized = true

proc execute_skill*() {.exportc.} =
  instance.initialized = true

proc persist_context*() {.exportc.} =
  instance.initialized = true

proc handle_message*() {.exportc.} =
  instance.initialized = true

proc render_canvas*() {.exportc.} =
  instance.initialized = true

proc claw_gateway_init*() {.exportc.} =
  instance.initialized = true

proc claw_route_message*() {.exportc.} =
  instance.initialized = true

proc claw_render_canvas*() {.exportc.} =
  instance.initialized = true

proc claw_execute_tool*() {.exportc.} =
  instance.initialized = true

proc claw_sandbox_policy*() {.exportc.} =
  instance.initialized = true

proc claw_daemon_init*() {.exportc.} =
  instance.initialized = true

