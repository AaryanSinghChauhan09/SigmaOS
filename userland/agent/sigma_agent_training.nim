# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_training.nim — Training data collector + fine-tuning pipeline
# Inspiration: Aider training, openclaw dataset, Claude Code RLHF
# Language: Nim — OOP via TrainingCollector + DatasetBuilder

import std/[os, json, strutils, tables, times, osproc, strformat]

# ── Training Sample Types ─────────────────────────────────────────────────────
type
  SampleQuality = enum Unknown, Good, Bad, Excellent

  TrainingSample = object
    id:          string
    timestamp:   string
    user_input:  string
    agent_output: string
    tools_used:  seq[string]
    success:     bool
    quality:     SampleQuality
    feedback:    string      # optional human feedback
    context:     string      # OS state (cwd, recent files)
    duration_ms: int

  Dataset = object
    name:      string
    samples:   seq[TrainingSample]
    version:   string
    created:   string
    stats:     DatasetStats

  DatasetStats = object
    total:    int
    good:     int
    bad:      int
    tools:    Table[string, int]  # tool usage counts
    avg_dur:  float

# ── JSONL Format (standard for LLM fine-tuning) ───────────────────────────────
proc to_chatml(s: TrainingSample): JsonNode =
  ## Convert to ChatML format for fine-tuning (OpenAI/llama.cpp compatible)
  let system_msg = %*{"role": "system", "content":
    "You are sigma-agent, the SigmaOS AI CLI assistant. Help users control their OS via natural language."}
  let user_msg   = %*{"role": "user",      "content": s.user_input}
  let assist_msg = %*{"role": "assistant", "content": s.agent_output}
  %*{"messages": [system_msg, user_msg, assist_msg],
     "quality": $s.quality, "tools": s.tools_used}

proc to_alpaca(s: TrainingSample): JsonNode =
  ## Alpaca format (instruction-input-output)
  %*{"instruction": "You are sigma-agent. Execute this OS command.",
     "input":       s.user_input,
     "output":      s.agent_output,
     "quality":     $s.quality}

# ── Collector ─────────────────────────────────────────────────────────────────
type TrainingCollector = object
  data_dir:   string
  session_id: string
  samples:    seq[TrainingSample]
  auto_save:  bool

proc new_collector(data_dir: string): TrainingCollector =
  createDir(data_dir)
  TrainingCollector(
    data_dir:   data_dir,
    session_id: $now().toUnixFloat.int,
    samples:    @[],
    auto_save:  true,
  )

proc record(c: var TrainingCollector, user_input, agent_output: string,
            tools: seq[string], success: bool, dur_ms: int) =
  let sample = TrainingSample(
    id:           c.session_id & "_" & $c.samples.len,
    timestamp:    $now(),
    user_input:   user_input,
    agent_output: agent_output,
    tools_used:   tools,
    success:      success,
    quality:      if success: Good else: Bad,
    feedback:     "",
    context:      getCurrentDir(),
    duration_ms:  dur_ms,
  )
  c.samples.add(sample)
  if c.auto_save: c.append_to_file(sample)

proc append_to_file(c: TrainingCollector, s: TrainingSample) =
  let path = c.data_dir / fmt"session_{c.session_id}.jsonl"
  let line = $s.to_chatml()
  var f = open(path, fmAppend)
  f.writeLine(line)
  f.close()

proc rate_last(c: var TrainingCollector, quality: SampleQuality, feedback = "") =
  if c.samples.len == 0: return
  c.samples[^1].quality  = quality
  c.samples[^1].feedback = feedback

# ── Dataset Builder ───────────────────────────────────────────────────────────
type DatasetBuilder = object
  sessions_dir: string
  output_dir:   string

proc new_builder(sessions_dir, output_dir: string): DatasetBuilder =
  createDir(output_dir)
  DatasetBuilder(sessions_dir: sessions_dir, output_dir: output_dir)

proc load_sessions(b: DatasetBuilder): seq[TrainingSample] =
  var all_samples: seq[TrainingSample]
  for _, path in walkDir(b.sessions_dir):
    if not path.endsWith(".jsonl"): continue
    for line in lines(path):
      if line.strip().len == 0: continue
      try:
        let j = parseJson(line)
        if j.hasKey("messages") and j["messages"].len >= 3:
          let msgs = j["messages"]
          all_samples.add TrainingSample(
            user_input:   msgs[1]["content"].getStr,
            agent_output: msgs[2]["content"].getStr,
            quality:      if j.getOrDefault("quality").getStr == "Excellent": Excellent
                          elif j.getOrDefault("quality").getStr == "Good": Good
                          else: Bad,
          )
      except: discard
  all_samples

proc compute_stats(samples: seq[TrainingSample]): DatasetStats =
  var stats = DatasetStats(tools: initTable[string, int]())
  stats.total = samples.len
  for s in samples:
    if s.quality in {Good, Excellent}: stats.good += 1
    elif s.quality == Bad: stats.bad += 1
    for t in s.tools_used:
      stats.tools[t] = stats.tools.getOrDefault(t, 0) + 1
    stats.avg_dur += s.duration_ms.float
  if stats.total > 0: stats.avg_dur /= stats.total.float
  stats

proc build_dataset(b: DatasetBuilder, name: string, min_quality = Good) =
  let samples = b.load_sessions()
  let filtered = samples.filterIt(it.quality >= min_quality)
  let stats    = compute_stats(filtered)

  echo fmt"Building dataset: {name}"
  echo fmt"  Total samples:   {samples.len}"
  echo fmt"  After filtering: {filtered.len}"
  echo fmt"  Good / Excellent: {stats.good}"

  # Write ChatML JSONL (for llama.cpp fine-tuning)
  let chatml_path = b.output_dir / fmt"{name}_chatml.jsonl"
  var f = open(chatml_path, fmWrite)
  for s in filtered: f.writeLine($s.to_chatml())
  f.close()
  echo fmt"  → {chatml_path}"

  # Write Alpaca JSONL
  let alpaca_path = b.output_dir / fmt"{name}_alpaca.jsonl"
  var fa = open(alpaca_path, fmWrite)
  for s in filtered: fa.writeLine($s.to_alpaca())
  fa.close()
  echo fmt"  → {alpaca_path}"

  # Write metadata
  let meta = %*{"name": name, "total": filtered.len, "good": stats.good,
                "version": "1.0", "created": $now()}
  writeFile(b.output_dir / fmt"{name}_meta.json", meta.pretty())
  echo fmt"✓ Dataset built: {filtered.len} samples"

# ── Built-in Training Data (seed dataset) ────────────────────────────────────
const SEED_SAMPLES = """
{"messages":[{"role":"system","content":"You are sigma-agent, the SigmaOS AI CLI assistant."},{"role":"user","content":"install sigma-edit"},{"role":"assistant","content":"✓ Installed sigma-edit 1.2.0"}],"quality":"Excellent"}
{"messages":[{"role":"system","content":"You are sigma-agent, the SigmaOS AI CLI assistant."},{"role":"user","content":"set dark mode"},{"role":"assistant","content":"✓ appearance.theme = zenith-dark"}],"quality":"Excellent"}
{"messages":[{"role":"system","content":"You are sigma-agent, the SigmaOS AI CLI assistant."},{"role":"user","content":"system info"},{"role":"assistant","content":"Σ SigmaOS v15.0 Zenith\nCPU: SovereignCPU  Memory: 256MB / 512MB  Uptime: 1h 23m"}],"quality":"Good"}
{"messages":[{"role":"system","content":"You are sigma-agent, the SigmaOS AI CLI assistant."},{"role":"user","content":"list /home/user"},{"role":"assistant","content":"Documents/  Downloads/  Code/  .config/  sigma-notes.md"}],"quality":"Good"}
{"messages":[{"role":"system","content":"You are sigma-agent, the SigmaOS AI CLI assistant."},{"role":"user","content":"show running processes"},{"role":"assistant","content":"PID    NAME\\n1      sigma-init\\n142    sigmad-health\\n143    sigmad-netd\\n200    sigma-sh"}],"quality":"Good"}
{"messages":[{"role":"system","content":"You are sigma-agent, the SigmaOS AI CLI assistant."},{"role":"user","content":"network status"},{"role":"assistant","content":"eth0: UP  192.168.1.100/24\\nwlan0: DOWN\\nDNS: 1.1.1.1, 9.9.9.9"}],"quality":"Good"}
{"messages":[{"role":"system","content":"You are sigma-agent, the SigmaOS AI CLI assistant."},{"role":"user","content":"accessibility high-contrast on"},{"role":"assistant","content":"✓ Accessibility: high-contrast = true"}],"quality":"Excellent"}
{"messages":[{"role":"system","content":"You are sigma-agent, the SigmaOS AI CLI assistant."},{"role":"user","content":"read /etc/hostname"},{"role":"assistant","content":"sigmaos"}],"quality":"Good"}
{"messages":[{"role":"system","content":"You are sigma-agent, the SigmaOS AI CLI assistant."},{"role":"user","content":"vpn connect work-vpn"},{"role":"assistant","content":"✓ Connected to work-vpn"}],"quality":"Good"}
{"messages":[{"role":"system","content":"You are sigma-agent, the SigmaOS AI CLI assistant."},{"role":"user","content":"disk usage"},{"role":"assistant","content":"Filesystem  Size  Used  Avail  Use%\\n/           20G   8.2G  11G    43%"}],"quality":"Good"}
"""

proc seed_dataset(data_dir: string) =
  ## Write built-in seed samples to bootstrap training
  createDir(data_dir)
  let path = data_dir / "seed_samples.jsonl"
  if not fileExists(path):
    writeFile(path, SEED_SAMPLES)
    echo fmt"✓ Seed dataset written: {path}"
  else:
    echo fmt"Seed dataset already exists: {path}"

proc sync_from_github*(data_dir: string) =
  ## Pull fresh training data from GitHub repo (wiki + agent conversations)
  const WIKI_PAGES = [
    "sigma-agent", "CLI-Reference", "Architecture-Overview",
    "Zenith-Desktop", "Getting-Started", "Security-Model"
  ]
  const REPO = "AaryanSinghChauhan09/SigmaOS"
  const RAW_BASE = "https://raw.githubusercontent.com/wiki/" & REPO
  createDir(data_dir)
  var synced = 0
  echo "Syncing training data from GitHub..."
  for page in WIKI_PAGES:
    let url = fmt"{RAW_BASE}/{page}.md"
    let (content, code) = execCmdEx(fmt"""curl -sf --max-time 10 "{url}" """)
    if code == 0 and content.len > 100:
      # Convert wiki page into training samples (instruction → summary pairs)
      let sample = %*{
        "messages": [
          %*{"role":"system",    "content":"You are sigma-agent, the SigmaOS AI CLI assistant."},
          %*{"role":"user",      "content":fmt"explain {page.replace(\"-\",\" \").toLowerAscii}"},
          %*{"role":"assistant", "content":content[0..<min(500, content.len)]}],
        "quality":  "Good",
        "source":   "github_wiki"}
      var f = open(data_dir / fmt"wiki_{page}.jsonl", fmWrite)
      f.writeLine($sample); f.close()
      synced += 1
  echo fmt"✓ Synced {synced}/{WIKI_PAGES.len} wiki pages as training samples"

proc compare_models*(data_dir, model_a, model_b: string) =
  ## A/B test two models on seed samples and compare pass rates
  let seed_path = data_dir / "seed_samples.jsonl"
  if not fileExists(seed_path):
    echo "✗ Run: sigma-agent train seed  first"; return

  echo fmt"\nComparing models:"
  echo fmt"  A: {model_a}"
  echo fmt"  B: {model_b}\n"

  var a_pass = 0; var b_pass = 0; var total = 0
  for line in lines(seed_path):
    if line.strip().len == 0: continue
    try:
      let j     = parseJson(line)
      let user  = j["messages"][1]["content"].getStr
      let ideal = j["messages"][2]["content"].getStr

      for (model, count) in [(model_a, addr a_pass), (model_b, addr b_pass)]:
        let env_var = fmt"SIGMA_LLM_MODEL={model.quoteShell}"
        let (out, _) = execCmdEx(fmt"{env_var} sigma-agent-core --once {user.quoteShell} 2>/dev/null")
        let ok = ideal[0..<min(20,ideal.len)].toLowerAscii in out.toLowerAscii
        if ok: count[] += 1

      total += 1
      if total >= 20: break  # quick A/B test on first 20 samples
    except: discard

  let pct_a = if total > 0: a_pass * 100 div total else: 0
  let pct_b = if total > 0: b_pass * 100 div total else: 0
  echo fmt"  Model A ({model_a}): {a_pass}/{total} ({pct_a}%)"
  echo fmt"  Model B ({model_b}): {b_pass}/{total} ({pct_b}%)"
  if   pct_b > pct_a: echo fmt"  → B wins by {pct_b-pct_a}%"
  elif pct_a > pct_b: echo fmt"  → A wins by {pct_a-pct_b}%"
  else:               echo "  → Tie"

# ── Fine-tuning command ───────────────────────────────────────────────────────
proc finetune_cmd*(args: seq[string]) =
  let home = getEnv("HOME", "/tmp")
  let data_dir   = home / ".cache/sigma/agent_training"
  let output_dir = home / ".cache/sigma/agent_datasets"

  if args.len == 0 or args[0] == "help":
    echo """sigma-agent train — Training data management

Usage:
  sigma-agent train seed          Write built-in seed dataset
  sigma-agent train build <name>  Build dataset from sessions
  sigma-agent train stats         Show training data statistics
  sigma-agent train rate good     Rate last interaction as good
  sigma-agent train rate bad      Rate last interaction as bad
  sigma-agent train list          List available datasets
"""
    return

  case args[0]
  of "seed":
    seed_dataset(data_dir)
    # Also copy v2 seed if present
    let v2_src = getAppDir() / "sigma_agent_seed_v2.jsonl"
    let v2_dst = data_dir / "seed_samples_v2.jsonl"
    if fileExists(v2_src) and not fileExists(v2_dst):
      copyFile(v2_src, v2_dst)
      echo fmt"✓ v2 seed dataset copied: {v2_dst}"
  of "build":
    let name = if args.len > 1: args[1] else: "sigma-agent-v1"
    let builder = new_builder(data_dir, output_dir)
    builder.build_dataset(name)
  of "stats":
    let builder = new_builder(data_dir, output_dir)
    let samples = builder.load_sessions()
    let stats   = compute_stats(samples)
    echo fmt"Training data statistics:"
    echo fmt"  Total samples:  {stats.total}"
    echo fmt"  Good/Excellent: {stats.good}"
    echo fmt"  Bad:            {stats.bad}"
    echo fmt"  Avg duration:   {stats.avg_dur:.0f}ms"
    echo fmt"  Top tools:"
    var top = toSeq(stats.tools.pairs)
    top.sort(proc(a,b:(string,int)):int = b[1]-a[1])
    for (t, c) in top[0..<min(5,top.len)]:
      echo fmt"    {t:<20} {c} uses"
  of "list":
    if dirExists(output_dir):
      for _, path in walkDir(output_dir):
        if path.endsWith(".jsonl"): echo "  " & path.extractFilename
    else: echo "No datasets built yet. Run: sigma-agent train build"
  of "rate":
    let quality = if args.len > 1 and args[1] == "bad": Bad
                  elif args.len > 1 and args[1] == "excellent": Excellent
                  else: Good
    echo fmt"✓ Last interaction rated as {quality}"
  of "sync":
    sync_from_github(data_dir)
  of "compare":
    let model_a = if args.len > 1: args[1] else: "tinyllama"
    let model_b = if args.len > 2: args[2] else: "sigma-agent-finetuned"
    compare_models(data_dir, model_a, model_b)
  else:
    echo fmt"Unknown train command: {args[0]}"
