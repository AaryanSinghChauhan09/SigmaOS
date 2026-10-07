# SPDX-License-Identifier: GPL-3.0-or-later
# SigmaOS Sovereign Notification Bus
# (`src/desktop/sigma_notification_bus.nim`)
# Nim module implementing org.freedesktop.Notifications protocol
# mediation, priority queueing, and desktop overlay dispatching.

import tables, strutils

type
  NotificationPriority* = enum
    npLow, npNormal, npCritical

  NotificationEvent* = object
    id*: uint32
    appName*: string
    summary*: string
    body*: string
    priority*: NotificationPriority
    expireTimeoutMs*: int32

  NotificationRouter* = object
    queue*: seq[NotificationEvent]
    nextId*: uint32
    dndMode*: bool

proc initNotificationRouter*(): NotificationRouter =
  result.queue = @[]
  result.nextId = 1
  result.dndMode = false

proc dispatchNotification*(router: var NotificationRouter, app, summary, body: string, prio: NotificationPriority): uint32 =
  let id = router.nextId
  inc(router.nextId)
  let event = NotificationEvent(
    id: id,
    appName: app,
    summary: summary,
    body: body,
    priority: prio,
    expireTimeoutMs: if prio == npCritical: -1 else: 5000
  )
  router.queue.add(event)
  result = id

proc toggleDnd*(router: var NotificationRouter, enabled: bool) =
  router.dndMode = enabled

proc getActiveNotifications*(router: NotificationRouter): seq[NotificationEvent] =
  result = @[]
  for n in router.queue:
    if not router.dndMode or n.priority == npCritical:
      result.add(n)
