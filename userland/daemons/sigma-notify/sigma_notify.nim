## SigmaOS: sigma_notify.h — Notification system
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

  # OOP Notification Base Class
  Notification* = ref object of RootObj
    id*: SigmaU32
    title*: array[64, char]
    titleLen*: SigmaUsize
    body*: array[256, char]
    bodyLen*: SigmaUsize

  # OOP Derived Subclass for Alert Notifications
  AlertNotification* = ref object of Notification
    urgency*: SigmaU8 // 0 = low, 1 = normal, 2 = critical

  # OOP Notification Manager
  NotificationManager* = ref object
    activeNotifications*: array[16, Notification]
    count*: SigmaUsize

# Base Class Methods
method init*(self: Notification, id: SigmaU32, title: string, body: string) {.base.} =
  self.id = id
  self.titleLen = if title.len > 63: 63 else: title.len
  for i in 0 ..< self.titleLen:
    self.title[i] = title[i]
  self.bodyLen = if body.len > 255: 255 else: body.len
  for i in 0 ..< self.bodyLen:
    self.body[i] = body[i]

method display*(self: Notification) {.base.} =
  # Freestanding mock display (normally prints to frame buffer)
  discard

# Derived Class Overrides
method init*(self: AlertNotification, id: SigmaU32, title: string, body: string, urgency: SigmaU8) =
  procCall self.Notification.init(id, title, body)
  self.urgency = urgency

method display*(self: AlertNotification) =
  # Display alert message with urgency highlights
  discard

# Manager Implementation
proc newNotificationManager*(): NotificationManager =
  NotificationManager(count: 0)

proc registerNotification*(mgr: NotificationManager, n: Notification) =
  if mgr.count < 16:
    mgr.activeNotifications[mgr.count] = n
    mgr.count += 1
    n.display()
