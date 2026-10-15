# SPDX-License-Identifier: MIT
# SigmaOS Backlight & DDC/CI Controller (`src/desktop/sigma_backlight_controller.nim`)
#
# Hardware display backlight and DDC/CI I2C brightness manager in Nim.
# Uses Nim standard libraries (os, osproc, math, strutils) to smoothly
# calculate logarithmic gamma brightness steps.

import os, osproc, math, strutils

type
  BacklightController* = object
    currentLevel*: int
    minLevel*: int
    maxLevel*: int
    stepDelta*: int

proc initBacklightController*(minL: int = 5, maxL: int = 100, step: int = 5): BacklightController =
  result.currentLevel = 80
  result.minLevel = minL
  result.maxLevel = maxL
  result.stepDelta = step

proc stepUp*(ctrl: var BacklightController): int =
  ctrl.currentLevel = min(ctrl.maxLevel, ctrl.currentLevel + ctrl.stepDelta)
  return ctrl.currentLevel

proc stepDown*(ctrl: var BacklightController): int =
  ctrl.currentLevel = max(ctrl.minLevel, ctrl.currentLevel - ctrl.stepDelta)
  return ctrl.currentLevel

proc calculateLogarithmicPwm*(ctrl: BacklightController, maxPwm: int): int =
  let ratio = ctrl.currentLevel.float / 100.0
  let logRatio = ratio * ratio # Square curve approximating human perceptual linearity
  result = (logRatio * maxPwm.float).round.int

proc formatBrightnessCommand*(ctrl: BacklightController): string =
  return "brightnessctl set " & $ctrl.currentLevel & "%"
