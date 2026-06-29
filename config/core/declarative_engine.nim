# SPDX-License-Identifier: GPL-2.0-or-later
# ==========================================================================
# SigmaOS: Declarative Config Engine (Nim)
# Replaces: config/core/DeclarativeEngine.cpp
# No stdlib, no third-party libs, OOP via Nim objects
# ==========================================================================

type
  ConfigValueKind* = enum
    cvString, cvInt, cvBool, cvList

  ConfigValue* = object
    case kind*: ConfigValueKind
    of cvString: strVal*: string
    of cvInt:    intVal*: int
    of cvBool:   boolVal*: bool
    of cvList:   listVal*: seq[string]

  ConfigEntry* = object
    key*: string
    value*: ConfigValue

  DeclarativeEngine* = object
    entries*: seq[ConfigEntry]
    count*:   int

proc newDeclarativeEngine*(): DeclarativeEngine =
  result.entries = newSeq[ConfigEntry]()
  result.count   = 0

proc addEntry*(engine: var DeclarativeEngine; key: string; value: ConfigValue) =
  engine.entries.add(ConfigEntry(key: key, value: value))
  engine.count += 1

proc findEntry*(engine: DeclarativeEngine; key: string): ptr ConfigEntry =
  for i in 0 ..< engine.entries.len:
    if engine.entries[i].key == key:
      return addr engine.entries[i]
  return nil

proc removeEntry*(engine: var DeclarativeEngine; key: string): bool =
  for i in 0 ..< engine.entries.len:
    if engine.entries[i].key == key:
      engine.entries.del(i)
      engine.count -= 1
      return true
  return false

proc applyConfig*(engine: DeclarativeEngine): bool =
  ## Applies all config entries (stub — real impl writes to kernel config tables)
  return engine.count > 0
