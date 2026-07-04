# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_learn.nim — Reinforcement / feedback learning module
# Records corrections, rates interactions, builds preference datasets,
# and generates fine-tuning scripts.
#
# Inspiration:
#   Claude Code RLHF pipeline    — human preference data
#   Aider correction loop        — "wrong, try this instead" → retrain
#   OpenClaw feedback system     — thumbs up/down per response
#   llama.cpp fine-tune guide    — LoRA fine-tuning from JSONL
#
# Flow:
#   User interaction → auto-record
#   User runs "sigma-agent learn correct/rate/reject" → annotate
#   "sigma-agent learn build" → exports preference JSONL
#   "sigma-agent learn finetune" → calls llama-finetune
#
# Language: Nim (stdlib only)

import std/[os, json, times, strutils, strformat, osproc, sequtils, tables]

# ── Types ─────────────────────────────────────────────────────────────────────
type
  Rating = enum RatingUnknown, RatingBad, RatingGood, RatingExcellent

  Interaction = object
    id:           string
    ts:           string
    user_input:   string
    agent_output: string
    correction:   string    # human-provided correction (empty if none)
    rating:       Rating
    tools_used:   seq[string]
    context_hint: string    # system context at time of interaction
    duration_ms:  int
    source:       string    # "repl" | "daemon" | "script"

  PreferencePair = object
    chosen:   string   # preferred response
    rejected: string   # worse response

# ── Storage paths ──────────────────────────────────────────────────────────────
proc data_dir(): string  = getEnv("HOME", "/tmp") / ".cache/sigma/agent_training"
proc pref_path(): string = data_dir() / "preferences.jsonl"
proc corr_path(): string = data_dir() / "corrections.jsonl"
proc hist_path(): string = getEnv("HOME", "/tmp") / ".sigma_agent_history"

# ── Serialisation ──────────────────────────────────────────────────────────────
proc to_chatml(i: Interaction): JsonNode =
  %*{"messages": [
    %*{"role":"system",    "content":"You are sigma-agent, the SigmaOS sovereign AI CLI assistant."},
    %*{"role":"user",      "content":i.user_input},
    %*{"role":"assistant", "content":i.agent_output}],
    "quality":   $i.rating,
    "source":    i.source,
    "duration":  i.duration_ms}

proc to_dpo(i: Interaction): JsonNode =
  ## Direct Preference Optimisation format (chosen / rejected pair)
  if i.correction.len == 0: return newJNull()
  %*{"prompt":   i.user_input,
     "chosen":   i.correction,   # human correction = preferred
     "rejected": i.agent_output}  # original = worse

proc to_alpaca(i: Interaction): JsonNode =
  %*{"instruction": "You are sigma-agent. Execute this OS command or request.",
     "input":       i.user_input,
     "output":      if i.correction.len > 0: i.correction else: i.agent_output,
     "quality":     $i.rating}

# ── Record an interaction ─────────────────────────────────────────────────────
proc record_interaction*(user_input, agent_output: string,
                         tools: seq[string] = @[],
                         duration_ms = 0,
                         source = "repl",
                         context_hint = "") =
  createDir(data_dir())
  let i = Interaction(
    id:           $now().toTime.toUnix & "_" & $user_input.hash.abs,
    ts:           $now(),
    user_input:   user_input,
    agent_output: agent_output,
    correction:   "",
    rating:       RatingUnknown,
    tools_used:   tools,
    duration_ms:  duration_ms,
    source:       source,
    context_hint: context_hint,
  )
  let path = data_dir() / fmt"session_{now().toTime.toUnix div 86400}.jsonl"
  var f = open(path, fmAppend)
  f.writeLine($i.to_chatml())
  f.close()

# ── Rate the last interaction ─────────────────────────────────────────────────
proc rate_last*(rating: Rating, correction = "") =
  createDir(data_dir())
  let session_file = data_dir() / fmt"session_{now().toTime.toUnix div 86400}.jsonl"
  if not fileExists(session_file): return

  # Read last line, annotate with rating/correction, write to preferences
  var lines: seq[string]
  for line in lines(session_file):
    if line.strip().len > 0: lines.add(line)
  if lines.len == 0: return

  try:
    var j = parseJson(lines[^1])
    j["quality"]    = %($rating)
    j["correction"] = %correction

    var f = open(pref_path(), fmAppend)
    f.writeLine($j)
    f.close()

    # If correction provided, also write DPO pair
    if correction.len > 0:
      let user_input = j["messages"][1]["content"].getStr
      let dpo = %*{"prompt": user_input, "chosen": correction,
                   "rejected": j["messages"][2]["content"].getStr}
      var cf = open(corr_path(), fmAppend)
      cf.writeLine($dpo)
      cf.close()

    echo fmt"✓ Rated interaction as {rating}"
    if correction.len > 0: echo fmt"  Correction recorded: {correction[0..<min(80, correction.len)]}"
  except CatchableError as e:
    echo fmt"✗ Could not rate: {e.msg}"

# ── Correction flow ────────────────────────────────────────────────────────────
proc record_correction*(wrong_output, correct_output, user_input: string) =
  createDir(data_dir())
  let dpo = %*{"prompt": user_input, "chosen": correct_output, "rejected": wrong_output,
               "ts": $now()}
  var f = open(corr_path(), fmAppend)
  f.writeLine($dpo)
  f.close()

  # Also write to preferences as "excellent" quality
  let chatml = %*{"messages":
    [%*{"role":"system",    "content":"You are sigma-agent, the SigmaOS AI CLI assistant."},
     %*{"role":"user",      "content":user_input},
     %*{"role":"assistant", "content":correct_output}],
    "quality": "Excellent", "source": "correction"}
  var pf = open(pref_path(), fmAppend)
  pf.writeLine($chatml)
  pf.close()

  echo fmt"✓ Correction recorded (will improve future responses)"
  echo fmt"  Input:    {user_input[0..<min(60, user_input.len)]}"
  echo fmt"  Correct:  {correct_output[0..<min(60, correct_output.len)]}"
  echo fmt"  Previous: {wrong_output[0..<min(60, wrong_output.len)]}"

# ── Dataset builder ────────────────────────────────────────────────────────────
type BuildStats = object
  total: int; good: int; excellent: int; with_correction: int; dpo_pairs: int

proc build_dataset*(name: string, min_rating = RatingGood,
                    include_dpo = true): BuildStats =
  createDir(data_dir())
  let output_dir = getEnv("HOME", "/tmp") / ".cache/sigma/agent_datasets"
  createDir(output_dir)

  var stats: BuildStats
  var chatml_f = open(output_dir / fmt"{name}_chatml.jsonl", fmWrite)
  var alpaca_f = open(output_dir / fmt"{name}_alpaca.jsonl", fmWrite)
  var dpo_f    = if include_dpo: open(output_dir / fmt"{name}_dpo.jsonl", fmWrite)
                 else: File(nil)

  # Collect all session files
  var all_lines: seq[string]
  for _, path in walkDir(data_dir()):
    if not path.endsWith(".jsonl") or path == corr_path(): continue
    for line in lines(path):
      if line.strip().len > 0: all_lines.add(line)

  # Include preference file
  if fileExists(pref_path()):
    for line in lines(pref_path()):
      if line.strip().len > 0: all_lines.add(line)

  for line in all_lines:
    try:
      let j = parseJson(line)
      if not j.hasKey("messages"): continue
      let quality_str = j.getOrDefault("quality").getStr("Unknown")
      let quality = case quality_str.toLowerAscii
        of "excellent": RatingExcellent
        of "good":      RatingGood
        of "bad":       RatingBad
        else:           RatingUnknown

      if quality < min_rating and quality != RatingUnknown: continue

      stats.total += 1
      if quality == RatingGood:      stats.good += 1
      if quality == RatingExcellent: stats.excellent += 1

      chatml_f.writeLine($j)

      # Build alpaca format
      let msgs = j["messages"]
      if msgs.len >= 3:
        let alpaca = %*{
          "instruction": "You are sigma-agent. Execute this OS command.",
          "input":       msgs[1]["content"].getStr,
          "output":      msgs[2]["content"].getStr,
          "quality":     quality_str}
        alpaca_f.writeLine($alpaca)
    except: discard

  # DPO pairs from corrections file
  if include_dpo and fileExists(corr_path()):
    for line in lines(corr_path()):
      if line.strip().len == 0: continue
      try:
        let j = parseJson(line)
        if j.hasKey("chosen") and j.hasKey("rejected"):
          if dpo_f != nil: dpo_f.writeLine($j)
          stats.dpo_pairs += 1
          stats.with_correction += 1
      except: discard

  chatml_f.close()
  alpaca_f.close()
  if dpo_f != nil: dpo_f.close()

  echo fmt"✓ Dataset '{name}' built:"
  echo fmt"  Total samples:    {stats.total}"
  echo fmt"  Good:             {stats.good}"
  echo fmt"  Excellent:        {stats.excellent}"
  echo fmt"  DPO pairs:        {stats.dpo_pairs}"
  echo fmt"  ChatML JSONL:     {output_dir}/{name}_chatml.jsonl"
  echo fmt"  Alpaca JSONL:     {output_dir}/{name}_alpaca.jsonl"
  if include_dpo: echo fmt"  DPO JSONL:        {output_dir}/{name}_dpo.jsonl"
  stats

# ── Fine-tune runner ──────────────────────────────────────────────────────────
proc run_finetune*(dataset_name: string, base_model: string, output_name: string) =
  let output_dir = getEnv("HOME", "/tmp") / ".cache/sigma/agent_datasets"
  let train_file = output_dir / fmt"{dataset_name}_chatml.jsonl"
  let model_dir  = getEnv("HOME", "/tmp") / ".cache/sigma/models"

  if not fileExists(train_file):
    echo fmt"✗ Dataset not found: {train_file}"
    echo fmt"  Run first: sigma-agent learn build {dataset_name}"
    return

  # Find llama-finetune
  let llama_ft = block:
    let candidates = ["llama-finetune", "/usr/bin/llama-finetune",
                      fmt"{model_dir}/../llama-finetune"]
    var found = ""
    for c in candidates:
      if fileExists(c) or execCmdEx(fmt"which {c}")[1] == 0: found = c; break
    found

  if llama_ft.len == 0:
    echo """✗ llama-finetune not found. Install with:
  sigma-pkg install llama-cpp
  # or
  curl -fsSL https://github.com/ggml-org/llama.cpp/releases/latest/download/llama-finetune -o /usr/bin/llama-finetune
  chmod +x /usr/bin/llama-finetune"""
    return

  let out_model = model_dir / fmt"{output_name}.gguf"
  let base_path = model_dir / base_model

  echo fmt"Σ Starting fine-tune:"
  echo fmt"  Base model:  {base_path}"
  echo fmt"  Train data:  {train_file}"
  echo fmt"  Output:      {out_model}"
  echo fmt"  (This may take 30–120 minutes depending on GPU/CPU)\n"

  let cmd = fmt"""{llama_ft} --model {base_path.quoteShell} \
  --train {train_file.quoteShell} \
  --output {out_model.quoteShell} \
  --lora-r 8 --lora-alpha 16 \
  --ctx 2048 --batch 4 --epochs 3 \
  --learning-rate 1e-4"""

  let (_, code) = execCmdEx(cmd)
  if code == 0:
    echo fmt"\n✓ Fine-tuned model saved: {out_model}"
    echo fmt"""  Use with: sigma-agent config set model {output_name}"""
  else:
    echo "\n✗ Fine-tuning failed. Check output above."

# ── Statistics ────────────────────────────────────────────────────────────────
proc show_stats*() =
  let d = data_dir()
  var total = 0; var good = 0; var excellent = 0; var bad = 0; var corrections = 0
  var tool_counts: Table[string, int]

  for _, path in walkDir(d):
    if not path.endsWith(".jsonl"): continue
    for line in lines(path):
      if line.strip().len == 0: continue
      try:
        let j = parseJson(line)
        total += 1
        case j.getOrDefault("quality").getStr.toLowerAscii
        of "excellent": excellent += 1
        of "good":      good += 1
        of "bad":       bad += 1
        # Count tool usage
        if j.hasKey("tools"):
          for t in j["tools"]: tool_counts[t.getStr] = tool_counts.getOrDefault(t.getStr,0)+1
      except: discard

  if fileExists(corr_path()):
    for line in lines(corr_path()):
      if line.strip().len > 0: corrections += 1

  echo fmt"""Σ Learning statistics:

  Total interactions:  {total}
  Excellent:           {excellent}
  Good:                {good}
  Bad:                 {bad}
  Corrections:         {corrections}
  Data directory:      {d}
"""

  if tool_counts.len > 0:
    var sorted_tools = toSeq(tool_counts.pairs).sortedByIt(-it[1])
    echo "  Top tools used:"
    for (tool, count) in sorted_tools[0..<min(8, sorted_tools.len)]:
      echo fmt"    {tool:<20} {count} times"

# ── CLI ────────────────────────────────────────────────────────────────────────
proc learn_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-agent learn — Reinforcement learning from feedback

Usage:
  sigma-agent learn rate good          Rate last interaction as good
  sigma-agent learn rate bad           Rate as bad
  sigma-agent learn rate excellent     Rate as excellent (prioritised in training)
  sigma-agent learn correct "<right>"  Provide the correct response
  sigma-agent learn build [name]       Build fine-tuning dataset
  sigma-agent learn finetune <model>   Run LoRA fine-tuning via llama.cpp
  sigma-agent learn stats              Show learning statistics
  sigma-agent learn export             Export dataset to current directory

How it works:
  1. Every sigma-agent interaction is recorded automatically
  2. You rate responses: good/bad/excellent
  3. "correct" records what the right answer should have been (DPO pair)
  4. "build" assembles ChatML + Alpaca + DPO JSONL datasets
  5. "finetune" calls llama-finetune to produce a custom GGUF model
  6. The new model can be loaded via: sigma-agent config set model <name>

Example workflow:
  sigma-agent "install sigma-edit"        # runs, maybe suboptimal
  sigma-agent learn rate good             # mark as good
  sigma-agent "set dark mode"             # wrong response
  sigma-agent learn correct "sigma-netctl settings set appearance theme zenith-dark"
  sigma-agent learn build sigma-v1        # build dataset
  sigma-agent learn finetune tinyllama-1.1b sigma-agent-v1   # fine-tune
"""
    return

  case args[0].toLowerAscii
  of "rate":
    let quality_str = if args.len > 1: args[1].toLowerAscii else: "good"
    let correction  = if args.len > 2: args[2..^1].join(" ") else: ""
    let rating = case quality_str
      of "excellent","great","perfect": RatingExcellent
      of "bad","wrong","no","terrible": RatingBad
      else:                             RatingGood
    rate_last(rating, correction)

  of "correct":
    if args.len < 2:
      echo "Usage: sigma-agent learn correct \"<correct response>\""
      return
    let correction = args[1..^1].join(" ")
    # Read last user input from history
    var last_user = ""
    if fileExists(hist_path()):
      let h = readFile(hist_path()).strip().splitLines()
      if h.len > 0: last_user = h[^1]
    if last_user.len == 0:
      echo "✗ No recent interaction found in history"
      return
    # Also record as rejection of whatever the agent said last
    record_correction("(unknown previous response)", correction, last_user)

  of "build","dataset":
    let name = if args.len > 1: args[1] else: fmt"sigma-agent-v{now().format(\"yyyyMMdd\")}"
    discard build_dataset(name, include_dpo=true)
    let od = getEnv("HOME", "/tmp") / ".cache/sigma/agent_datasets"
    echo fmt"""
Fine-tuning commands:
  # With llama.cpp:
  llama-finetune --model tinyllama.gguf --train {od}/{name}_chatml.jsonl --output sigma-agent-v1.gguf

  # Quick shortcut:
  sigma-agent learn finetune tinyllama-1.1b sigma-agent-v1
"""

  of "finetune","fine-tune","train":
    let base  = if args.len > 1: args[1] else: "tinyllama-1.1b-chat-q4_0.gguf"
    let name  = if args.len > 2: args[2] else: "sigma-agent-finetuned"
    let dname = if args.len > 3: args[3] else: fmt"sigma-agent-v{now().format(\"yyyyMMdd\")}"
    # Build dataset first if needed
    let od = getEnv("HOME", "/tmp") / ".cache/sigma/agent_datasets"
    if not fileExists(od / fmt"{dname}_chatml.jsonl"):
      discard build_dataset(dname)
    run_finetune(dname, base, name)

  of "stats","status":
    show_stats()

  of "export":
    let od = getEnv("HOME", "/tmp") / ".cache/sigma/agent_datasets"
    let name = fmt"sigma-agent-export-{now().format(\"yyyyMMdd\")}"
    discard build_dataset(name)
    echo fmt"✓ Exported to: {od}/{name}_*.jsonl"

  of "clear","reset":
    echo "This will delete all training data. Are you sure? (yes/no)"
    let confirm = stdin.readLine().strip().toLowerAscii
    if confirm == "yes":
      removeDir(data_dir())
      createDir(data_dir())
      echo "✓ Training data cleared"
    else: echo "Cancelled"

  else:
    echo fmt"Unknown learn command: {args[0]}. Run 'sigma-agent learn' for usage."
