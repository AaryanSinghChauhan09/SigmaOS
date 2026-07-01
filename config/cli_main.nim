# SPDX-License-Identifier: GPL-2.0-or-later
# ==========================================================================
# SigmaOS: Config CLI Main (Nim)
# Replaces: config/cli_main.cpp
# ==========================================================================

import config/core/declarative_engine
import config/core/generation_manager

type
  CliCommand* = enum
    cmdApply, cmdRollback, cmdList, cmdGet, cmdSet, cmdUnknown

  SigmaConfigCLI* = object
    engine*:  DeclarativeEngine
    genMgr*:  GenerationManager

proc newCLI*(): SigmaConfigCLI =
  result.engine = newDeclarativeEngine()
  result.genMgr = newGenerationManager()

proc parseCommand*(input: string): CliCommand =
  if input == "apply":
    return cmdApply
  elif input == "rollback":
    return cmdRollback
  elif input == "list":
    return cmdList
  elif input == "get":
    return cmdGet
  elif input == "set":
    return cmdSet
  else:
    return cmdUnknown

proc dispatch*(cli: var SigmaConfigCLI; cmd: CliCommand; args: seq[string]): bool =
  case cmd
  of cmdApply:
    return cli.engine.applyConfig()
  of cmdRollback:
    return cli.genMgr.rollback()
  of cmdList:
    for entry in cli.engine.entries:
      discard entry  # Stub: would print entries
    return true
  of cmdGet:
    if args.len > 0:
      let found = cli.engine.findEntry(args[0])
      return found != nil
    return false
  of cmdSet:
    if args.len > 1:
      cli.engine.addEntry(args[0], ConfigValue(kind: cvString, strVal: args[1]))
      return true
    return false
  of cmdUnknown:
    return false
