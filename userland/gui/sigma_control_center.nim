## sigma_control_center.nim — Zenith Desktop Control Center (Sigma Desktop UX)
## Language: Nim (freestanding OOP; no stdlib; no third-party)
## OOP: DesktopWidget (base), PanelWidget/WindowManager/AppLauncher (derived)
## Specification: wiki_repo/Desktop-UX.md
{.push raises: [].}

# ══════════════════════════════════════════════════════════════
#  § 1. Primitive types
# ══════════════════════════════════════════════════════════════

type
  SigmaU8*    = uint8
  SigmaU16*   = uint16
  SigmaU32*   = uint32
  SigmaI32*   = int32
  SigmaUsize* = uint
  SigmaBool*  = bool
  Pixel*      = SigmaU32  ## ARGB8888

# ══════════════════════════════════════════════════════════════
#  § 2. Rect — axis-aligned bounding box
# ══════════════════════════════════════════════════════════════

type
  Rect* = object
    x*, y*:  SigmaI32
    w*, h*:  SigmaU16

proc contains*(r: Rect; px, py: SigmaI32): SigmaBool =
  px >= r.x and px < (r.x + r.w.SigmaI32) and
  py >= r.y and py < (r.y + r.h.SigmaI32)

# ══════════════════════════════════════════════════════════════
#  § 3. Event system (no dynamic dispatch, manual vtable)
# ══════════════════════════════════════════════════════════════

type
  EventKind* = enum
    evMouseDown, evMouseUp, evMouseMove, evKeyDown, evKeyUp, evResize, evPaint

  Event* = object
    kind*: EventKind
    x*, y*: SigmaI32   # Mouse position or resize dimensions
    key*:   SigmaU16   # Scancode for keyboard events

# ══════════════════════════════════════════════════════════════
#  § 4. Framebuffer: hand-rolled pixel buffer
# ══════════════════════════════════════════════════════════════

const SCREEN_W = 1920
const SCREEN_H = 1080

type
  Framebuffer* = object
    pixels*: array[SCREEN_W * SCREEN_H, Pixel]
    width*:  SigmaU16
    height*: SigmaU16

proc initFramebuffer*(fb: var Framebuffer) =
  fb.width  = SCREEN_W
  fb.height = SCREEN_H
  var i: SigmaUsize = 0
  while i < (SCREEN_W * SCREEN_H).SigmaUsize:
    fb.pixels[i] = 0xFF1A1A2E'u32  # Deep navy background
    i += 1

proc setPixel*(fb: var Framebuffer; x, y: SigmaI32; c: Pixel) =
  if x < 0 or y < 0 or x.uint >= SCREEN_W.uint or y.uint >= SCREEN_H.uint: return
  fb.pixels[y.uint * SCREEN_W.uint + x.uint] = c

proc fillRect*(fb: var Framebuffer; r: Rect; c: Pixel) =
  var dy: SigmaI32 = 0
  while dy < r.h.SigmaI32:
    var dx: SigmaI32 = 0
    while dx < r.w.SigmaI32:
      fb.setPixel(r.x + dx, r.y + dy, c)
      dx += 1
    dy += 1

# ══════════════════════════════════════════════════════════════
#  § 5. OOP: DesktopWidget (abstract base)
# ══════════════════════════════════════════════════════════════

type
  DesktopWidget* = ref object of RootObj
    bounds*:   Rect
    visible*:  SigmaBool
    focused*:  SigmaBool
    dirty*:    SigmaBool
    widgetId*: SigmaU32

method paint*(self: DesktopWidget; fb: var Framebuffer) {.base.} =
  ## Override in sub-widgets.
  discard

method handleEvent*(self: DesktopWidget; ev: Event): SigmaBool {.base.} =
  false

proc show*(self: DesktopWidget) = self.visible = true
proc hide*(self: DesktopWidget) = self.visible = false
proc markDirty*(self: DesktopWidget) = self.dirty = true

# ══════════════════════════════════════════════════════════════
#  § 6. PanelWidget — top status bar (derived from DesktopWidget)
# ══════════════════════════════════════════════════════════════

const PANEL_H: SigmaU16 = 32

type
  PanelWidget* = ref object of DesktopWidget
    accentColor*: Pixel

proc newPanelWidget*(screenW: SigmaU16): PanelWidget =
  result = PanelWidget(
    bounds:      Rect(x: 0, y: 0, w: screenW, h: PANEL_H),
    visible:     true,
    dirty:       true,
    widgetId:    0,
    accentColor: 0xFF6C63FF'u32,  # Sigma Purple
  )

method paint*(self: PanelWidget; fb: var Framebuffer) =
  ## Draw a gradient-ish header bar using solid accent + glass tint.
  fb.fillRect(self.bounds, self.accentColor)
  # Subtle highlight line
  var dx: SigmaI32 = 0
  while dx < self.bounds.w.SigmaI32:
    fb.setPixel(dx, 0, 0xFFFFFFFF'u32)  # Top 1-pixel white line
    dx += 1
  self.dirty = false

# ══════════════════════════════════════════════════════════════
#  § 7. AppLauncher — dock widget
# ══════════════════════════════════════════════════════════════

const MAX_DOCK_ENTRIES = 16
const DOCK_ICON_SIZE: SigmaU16 = 48
const DOCK_H: SigmaU16 = 64

type
  DockEntry* = object
    labelChars*: array[32, char]
    labelLen*:   SigmaUsize
    iconColor*:  Pixel
    running*:    SigmaBool

  AppLauncher* = ref object of DesktopWidget
    entries*: array[MAX_DOCK_ENTRIES, DockEntry]
    count*:   SigmaUsize

proc newAppLauncher*(screenW, screenH: SigmaU16): AppLauncher =
  result = AppLauncher(
    bounds:  Rect(x: 0, y: (screenH - DOCK_H).SigmaI32, w: screenW, h: DOCK_H),
    visible: true,
    dirty:   true,
    widgetId: 1,
  )

proc addEntry*(dock: AppLauncher; label: openarray[char]; col: Pixel) =
  if dock.count >= MAX_DOCK_ENTRIES: return
  let e: DockEntry = DockEntry(iconColor: col, running: false)
  var i: SigmaUsize = 0
  while i < label.len.SigmaUsize and i < 31.SigmaUsize:
    dock.entries[dock.count].labelChars[i] = label[i]
    i += 1
  dock.entries[dock.count].labelLen = i
  dock.count += 1

method paint*(self: AppLauncher; fb: var Framebuffer) =
  ## Draw translucent dock bar and each icon as colored rounded square.
  fb.fillRect(self.bounds, 0xCC0D0D1A'u32)  # Semi-opaque dark
  var i: SigmaUsize = 0
  while i < self.count:
    let ex = (i * (DOCK_ICON_SIZE + 12).SigmaUsize).SigmaI32 + 16
    let ey = self.bounds.y + 8
    let iconRect = Rect(x: ex, y: ey, w: DOCK_ICON_SIZE, h: DOCK_ICON_SIZE)
    fb.fillRect(iconRect, self.entries[i].iconColor)
    i += 1
  self.dirty = false

# ══════════════════════════════════════════════════════════════
#  § 8. WindowManager — compositor (composition of widgets)
# ══════════════════════════════════════════════════════════════

const MAX_WINDOWS = 32

type
  WindowManager* = ref object
    panel*:   PanelWidget
    dock*:    AppLauncher
    fb*:      Framebuffer
    running*: SigmaBool

proc newWindowManager*(): WindowManager =
  result = WindowManager(running: true)
  result.fb.initFramebuffer()
  result.panel = newPanelWidget(SCREEN_W.uint16)
  result.dock  = newAppLauncher(SCREEN_W.uint16, SCREEN_H.uint16)

proc paintAll*(wm: WindowManager) =
  wm.panel.paint(wm.fb)
  wm.dock.paint(wm.fb)

proc dispatchEvent*(wm: WindowManager; ev: Event): SigmaBool =
  discard wm.panel.handleEvent(ev)
  discard wm.dock.handleEvent(ev)
  true

# ══════════════════════════════════════════════════════════════
#  § 9. Unit tests
# ══════════════════════════════════════════════════════════════

proc testDesktopUX*(): bool =
  let wm = newWindowManager()
  # Populate dock
  wm.dock.addEntry(['T', 'e', 'r', 'm'], 0xFF00BFFF'u32)
  wm.dock.addEntry(['F', 'i', 'l', 'e', 's'], 0xFFFF8C00'u32)
  if wm.dock.count != 2: return false
  # Paint to framebuffer
  wm.paintAll()
  # Check panel pixel
  let panelPixel = wm.fb.pixels[0]  # Top-left corner
  if panelPixel != 0xFF6C63FF'u32: return false   # Accent color
  true
