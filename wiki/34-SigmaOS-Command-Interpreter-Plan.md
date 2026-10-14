# Idea #34: SigmaOS Command Interpreter Development Plan

## Overview
This document specifies the SigmaOS Command Interpreter Development Plan and 17-PR Sequence Gateway Suite.
It defines product and compatibility rules, a 17-PR sequence from initial safe command runner baseline to a documented POSIX subset, periodic maintenance cadences, component rules, PR review/merge gates, and rollout/maturity criteria.

## Product and Compatibility Rules
1. **Safe Native Command Runner First:** Begin with a safe native command runner, then add a documented POSIX subset in phases.
2. **Parse Before Executing:** Syntax errors are identified before any part of the command executes.
3. **Preserve Arguments Exactly:** Structured argument arrays without re-splitting.
4. **AST Architecture:** Lexer, parser, expansion, command resolution, and execution are decoupled.
5. **Distinct Interactive & Script Modes:** Script mode avoids prompts, history expansion, or line editing.
6. **Untrusted Input Defense:** Strict bounds on command length, tokens, AST depth, expansion size, environment size, and history size.
7. **Privileged Separation:** Built-ins that change system state must use typed interfaces with permissions.

## Proposed 17-PR Sequence Gateway Suite
- **PR 01:** `docs(shell): define command interpreter goals and compatibility profile`
- **PR 02:** `refactor(shell): split lexer parser expander and executor modules`
- **PR 03:** `test(shell): add golden lexer and parser regression suite`
- **PR 04:** `feat(shell): implement command lookup and process execution contract`
- **PR 05:** `feat(shell): add variables assignments and safe quoting`
- **PR 06:** `feat(shell): implement POSIX-style redirection primitives`
- **PR 07:** `feat(shell): implement pipelines with bounded process and pipe lifecycle`
- **PR 08:** `feat(shell): define exit status and command-list operators`
- **PR 09:** `feat(shell): add environment and PATH built-ins`
- **PR 10:** `feat(shell): add script mode and startup policy`
- **PR 11:** `feat(shell): add control-flow subset for shell scripts`
- **PR 12:** `feat(shell): add interactive line editor and bounded history`
- **PR 13:** `feat(shell): add foreground job control and signal handling`
- **PR 14:** `feat(shell): add command completion and help metadata`
- **PR 15:** `security(shell): enforce parser and expansion resource limits`
- **PR 16:** `test(shell): add POSIX subset and Linux BSD interoperability matrix`
- **PR 17:** `docs(shell): publish command reference and maturity evidence`

## Periodic Maintenance Cadences
- **Every PR:** `ci(shell): run shell-specific checks for changed code`
- **Weekly:** `test(shell): review fuzz crashes and parser regressions` & `docs(shell): reconcile feature status with implementation`
- **Monthly:** `security(shell): review command execution and expansion boundaries`, `test(shell): run script compatibility corpus`, & `maint(shell): review terminal, signal, and child-process cleanup`
- **Quarterly:** `docs(shell): review POSIX and reference-shell alignment`
- **Release Candidate:** `release(shell): certify shell support claims`
- **Post-Incident:** `fix(shell): prevent recurrence of <bug>`

## Maturity Gates
- **Prototype:** Native command runner baseline with AST parser, simple built-ins, and resource limits governor.
- **Integrated:** OS process and file descriptor mechanisms wired for pipelines, redirection, and script execution.
- **Supported:** Repeatable regression/PTY testing and certified compatibility profile matching observed behavior.
