# SPDX-License-Identifier: GPL-3.0-or-later
# SigmaOS Sovereign Waybar & QuickShell IPC Streamer
# (`src/desktop/sigma_waybar_streamer.nim`)
# Nim module for sub-millisecond telemetry formatting and UNIX domain
# socket streaming to Waybar, QuickShell, and desktop status bars.

import json, strutils

type
  BarWidgetType* = enum
    bwtCpu, bwtMemory, bwtGpu, bwtNetwork, bwtBattery, bwtClock, bwtAudio

  WidgetTelemetry* = object
    widget*: BarWidgetType
    value*: float
    unit*: string
    text*: string
    tooltip*: string
    class*: string

  WaybarFrameBuffer* = object
    widgets*: seq[WidgetTelemetry]
    frameId*: int64

proc initFrameBuffer*(): WaybarFrameBuffer =
  result.widgets = @[]
  result.frameId = 0

proc addWidget*(fb: var WaybarFrameBuffer, widget: WidgetTelemetry) =
  fb.widgets.add(widget)

proc renderJsonFrame*(fb: WaybarFrameBuffer): string =
  var jobj = newJObject()
  for w in fb.widgets:
    jobj[$w.widget] = %* {
      "text": w.text,
      "tooltip": w.tooltip,
      "class": w.class,
      "value": w.value
    }
  result = $jobj

proc createCpuWidget*(usage: float): WidgetTelemetry =
  result.widget = bwtCpu
  result.value = usage
  result.unit = "%"
  result.text = " " & $int(usage) & "%"
  result.tooltip = "CPU Usage: " & formatFloat(usage, ffDecimal, 1) & "%"
  result.class = if usage > 85.0: "critical" elif usage > 50.0: "warning" else: "normal"
