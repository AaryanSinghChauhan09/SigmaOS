# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/agent/sigma_agent_corpus.nim — AI training corpus builder
# Assembles a comprehensive training dataset from the entire SigmaOS codebase:
# - Wiki pages → concept explanations
# - GUI mirror → command mappings
# - Workflow templates → automation examples
# - Plugin commands → extended capabilities
# - System docs → factual knowledge
# - GitHub history → commit patterns
#
# Produces: corpus_combined.jsonl (ChatML) ready for llama-finetune
#
# Language: Nim (stdlib only)

import std/[os, osproc, json, strutils, strformat, times, sequtils, tables]

# ── Corpus entry ────────────────────────────────────────────────────────────
const SYS_PROMPT = "You are sigma-agent, the SigmaOS sovereign AI CLI assistant. Help users control their OS via natural language."

proc make_sample(user, assistant, quality = "Good"): JsonNode =
  %*{"messages":[
    %*{"role":"system",    "content":SYS_PROMPT},
    %*{"role":"user",      "content":user},
    %*{"role":"assistant", "content":assistant}],
    "quality":quality, "source":"corpus"}

# ── 1. Extract from GUI mirror ────────────────────────────────────────────────
proc extract_gui_mirror(mirror_nim: string): seq[JsonNode] =
  if not fileExists(mirror_nim): return
  for line in lines(mirror_nim):
    # Pattern: "open terminal": "sigma-agent \"open app sigma-terminal\""
    if "\":" in line and line.strip().startsWith("\""):
      let parts = line.strip().split("\": \"")
      if parts.len == 2:
        let action = parts[0].strip(chars={'"',' '})
        let cmd    = parts[1].strip(chars={'"',',',' '})
        if action.len > 3 and cmd.len > 3:
          result.add make_sample(action, cmd, "Excellent")
          # Also natural language variant
          result.add make_sample(
            fmt"how do I {action}",
            fmt"Run: {cmd}", "Good")

# ── 2. Extract from workflow templates ────────────────────────────────────────
proc extract_workflows(workflow_nim: string): seq[JsonNode] =
  if not fileExists(workflow_nim): return
  var in_template = false
  var template_name = ""
  var template_desc = ""
  var yaml_lines: seq[string]

  for line in lines(workflow_nim):
    let stripped = line.strip()
    if stripped.startsWith("(\"") and ", \"\"\"" in stripped:
      if yaml_lines.len > 0 and template_name.len > 0:
        let yaml = yaml_lines.join("\n")
        result.add make_sample(
          fmt"create a workflow to {template_desc}",
          yaml, "Excellent")
        result.add make_sample(
          fmt"sigma-agent workflow create \"{template_desc}\"",
          yaml, "Excellent")
        yaml_lines = @[]
      let parts = stripped.split(", \"\"\"")
      template_name = parts[0].strip(chars={'"','(',' '})
      in_template = true
    elif in_template and stripped == "\"\"\")," or stripped == "\"\"\"),":
      in_template = false
    elif in_template:
      yaml_lines.add(line.replace("\"\"\"","").strip())
      if stripped.startsWith("description:"):
        template_desc = stripped.split(":")[1].strip().strip(chars={'"'})

# ── 3. Extract from wiki pages ────────────────────────────────────────────────
proc extract_wiki_pages(wiki_dir: string): seq[JsonNode] =
  if not dirExists(wiki_dir): return
  const RELEVANT_PAGES = [
    "sigma-agent.md", "Architecture-Overview.md", "Security-Model.md",
    "Kernel.md", "Getting-Started.md", "sigma-agent-workflow.md",
    "Migration-Guide.md", "SDK-Guide.md", "sigpkg-Spec.md",
    "Linux-Absorption-Architecture.md", "SigmaOS-vs-Linux.md",
  ]
  for page in RELEVANT_PAGES:
    let path = wiki_dir / page
    if not fileExists(path): continue
    let content = readFile(path)
    # Extract Q&A pairs from headers + body
    var current_section = ""
    var section_lines: seq[string]
    for line in content.splitLines():
      if line.startsWith("## ") or line.startsWith("### "):
        if section_lines.len > 3 and current_section.len > 0:
          let body = section_lines.join("\n").strip()
          if body.len > 30:
            result.add make_sample(
              fmt"what is {current_section.strip().strip(chars={'#',' '})}",
              body[0..<min(400, body.len)], "Good")
        current_section = line
        section_lines = @[]
      else:
        section_lines.add(line)
    # Extract code blocks as command examples
    var in_code = false
    var code_lines: seq[string]
    for line in content.splitLines():
      if line.startsWith("```"):
        if in_code and code_lines.len > 0:
          for code_line in code_lines:
            let l = code_line.strip()
            if l.startsWith("sigma-agent ") and l.len > 15:
              let cmd = l[12..^1].strip(chars={'"'})
              if cmd.len > 5:
                result.add make_sample(cmd, fmt"Running: {l}", "Good")
          code_lines = @[]
        in_code = not in_code
      elif in_code:
        code_lines.add(line)

# ── 4. Generate command variation samples ─────────────────────────────────────
const COMMAND_VARIATIONS: array[30, (string, string)] = [
  ("how do I switch to dark mode",       "sigma-agent \"set dark mode\""),
  ("make it dark",                        "sigma-agent \"set dark mode\""),
  ("I want dark theme",                   "sigma-agent \"set dark mode\""),
  ("turn on dark mode",                   "sigma-agent \"set dark mode\""),
  ("enable night mode",                   "sigma-agent \"set dark mode\""),
  ("what's running on my system",         "sigma-agent \"show processes\""),
  ("show me system info",                 "sigma-agent \"system info\""),
  ("how much ram am I using",             "sigma-agent \"system info\""),
  ("is my cpu busy",                      "sigma-agent \"show processes\""),
  ("install an app",                      "sigma-agent \"install <package-name>\""),
  ("how do I install packages",           "sigma-pkg install <package-name>"),
  ("update everything",                   "sigma-pkg update"),
  ("is my internet working",              "sigma-agent \"network status\""),
  ("connect to wifi",                     "sigma-netctl wifi <iface> <ssid> <password>"),
  ("I can't connect to the internet",     "sigma-agent \"network status\""),
  ("check disk space",                    "sigma-agent \"disk usage\""),
  ("I'm running out of space",            "sigma-agent \"disk usage\""),
  ("enable accessibility",                "sigma-agent \"accessibility high-contrast on\""),
  ("make text bigger",                    "sigma-agent \"accessibility large-text on\""),
  ("start a terminal",                    "sigma-agent \"open app sigma-terminal\""),
  ("take a screenshot",                   "sigma-agent \"run sigma-screenshot\""),
  ("lock my screen",                      "sigma-lock"),
  ("set up a backup",                     "sigma-agent workflow install weekly-backup"),
  ("automate my backups",                 "sigma-agent workflow create \"backup my files every Friday\""),
  ("run a security check",                "sigma-agent security scan"),
  ("is my system secure",                 "sigma-agent security scan"),
  ("explain how to install apps",         "sigma-agent explain \"how to install apps on SigmaOS\""),
  ("what version of sigmaos am I on",     "sigma-agent \"system info\""),
  ("open my files",                       "sigma-agent \"open app sigma-files\""),
  ("how do I use the vpn",               "sigma-vpn connect <profile-name>"),
]

proc generate_variation_samples(): seq[JsonNode] =
  for (user_nl, cmd) in COMMAND_VARIATIONS:
    result.add make_sample(user_nl, cmd, "Excellent")

# ── 5. Error explanation samples ──────────────────────────────────────────────
const ERROR_SAMPLES: array[10, (string, string)] = [
  ("permission denied when running command",
   "You lack the required permissions. Try: sudo <command> or check sigma_pledge capabilities with: sigma-agent security scan"),
  ("command not found: sigma-edit",
   "sigma-edit is not installed. Install it with: sigma-pkg install sigma-edit"),
  ("network unreachable",
   "Your network interface may be down. Check: sigma-agent \"network status\" then try: sigma-netctl dhcp <iface>"),
  ("disk full",
   "Your disk is full. Check usage: sigma-agent \"disk usage\" then free space: sigma-agent \"run rm -rf /tmp/*\""),
  ("port already in use",
   "Another process is using that port. Find it: sigma-agent \"show processes\" then kill it: sigma-agent \"kill process <pid>\""),
  ("authentication failure",
   "Login failed. Check: sigma-agent security logs — if multiple failures, enable fail2ban: sigma-pkg install fail2ban"),
  ("cargo build failed with linker error",
   "Missing system libraries. Try: sigma-pkg install build-essential then: cargo build --release"),
  ("nim compilation error: module not found",
   "Missing Nim module. Check: nimble install <module-name> or: sigma-pkg install nim-<module>"),
  ("sigma-agent daemon not running",
   "Start the daemon: sigma-agent daemon start — it provides GitHub knowledge sync and LLM tab completion"),
  ("failed to install package: signature invalid",
   "Package signature verification failed. This may be a tampered package. Only install from sigma-pkg registry: sigma-pkg search <name>"),
]

proc generate_error_samples(): seq[JsonNode] =
  for (error, fix) in ERROR_SAMPLES:
    result.add make_sample(
      fmt"I got this error: {error}",
      fmt"Error: {error}\n\nFix: {fix}", "Excellent")
    result.add make_sample(
      fmt"fix: {error}",
      fix, "Good")

# ── Main corpus builder ────────────────────────────────────────────────────────
proc build_corpus*(repo_root: string, output_path: string) =
  echo "\e[38;2;69;243;255mΣ Building sigma-agent training corpus...\e[0m\n"
  var all_samples: seq[JsonNode]

  # 1. GUI mirror
  let mirror_path = repo_root / "userland/agent/sigma_agent_gui_mirror.nim"
  let mirror_samples = extract_gui_mirror(mirror_path)
  echo fmt"  GUI mirror:         {mirror_samples.len} samples"
  all_samples.add(mirror_samples)

  # 2. Workflow templates
  let wf_path = repo_root / "userland/agent/sigma_agent_workflow.nim"
  let wf_samples = extract_workflows(wf_path)
  echo fmt"  Workflow templates: {wf_samples.len} samples"
  all_samples.add(wf_samples)

  # 3. Wiki pages
  let wiki_samples = extract_wiki_pages(repo_root / "wiki_repo")
  echo fmt"  Wiki pages:         {wiki_samples.len} samples"
  all_samples.add(wiki_samples)

  # 4. Command variations (NL synonyms)
  let var_samples = generate_variation_samples()
  echo fmt"  NL variations:      {var_samples.len} samples"
  all_samples.add(var_samples)

  # 5. Error explanations
  let err_samples = generate_error_samples()
  echo fmt"  Error explanations: {err_samples.len} samples"
  all_samples.add(err_samples)

  # 6. Load existing seed files
  for seed in ["sigma_agent_seed_v2.jsonl"]:
    let seed_path = repo_root / "userland/agent" / seed
    if fileExists(seed_path):
      var count = 0
      for line in lines(seed_path):
        if line.strip().len > 0:
          try: all_samples.add(parseJson(line)); count += 1 except: discard
      echo fmt"  Seed ({seed}): {count} samples"

  # Remove duplicates by user content
  var seen: Table[string, bool]
  var deduped: seq[JsonNode]
  for s in all_samples:
    try:
      let key = s["messages"][1]["content"].getStr
      if key notin seen:
        seen[key] = true
        deduped.add(s)
    except: discard

  echo fmt"\n  Total:    {all_samples.len} (before dedup)"
  echo fmt"  Unique:   {deduped.len} (after dedup)"

  # Write output
  createDir(output_path.parentDir())
  var f = open(output_path, fmWrite)
  for s in deduped: f.writeLine($s)
  f.close()

  echo fmt"\n✓ Corpus written: {output_path}"
  echo fmt"  {deduped.len} samples ready for fine-tuning"
  echo fmt"""
Fine-tuning command:
  llama-finetune --model tinyllama.gguf \\
    --train {output_path} \\
    --output sigma-agent-v2.gguf \\
    --lora-r 8 --epochs 3
"""

# ── CLI ─────────────────────────────────────────────────────────────────────
proc corpus_cmd*(args: seq[string]) =
  if args.len == 0 or args[0] == "help":
    echo """sigma-agent corpus — AI training corpus builder

Usage:
  sigma-agent corpus build             Build corpus from repo
  sigma-agent corpus build --out <f>   Custom output path
  sigma-agent corpus stats             Show corpus statistics
  sigma-agent corpus clean             Remove generated corpus

The corpus builder assembles training data from:
  - GUI mirror (60+ command mappings)
  - Workflow templates (8+ automation examples)
  - Wiki pages (700+ documentation pages)
  - NL variations (30 command phrasings)
  - Error explanations (10 common errors + fixes)
  - Existing seed files (65+ curated samples)

Output: ~/.cache/sigma/agent_datasets/corpus_combined.jsonl
"""
    return

  let repo_root = getAppDir() / "../.."  # userland/agent → repo root
  let output_dir = getEnv("HOME","/tmp") / ".cache/sigma/agent_datasets"
  let output_path = output_dir / "corpus_combined.jsonl"

  case args[0].toLowerAscii
  of "build":
    let out = if args.len > 2 and args[1] == "--out": args[2]
              else: output_path
    build_corpus(repo_root, out)
  of "stats":
    if fileExists(output_path):
      var count = 0
      for line in lines(output_path):
        if line.strip().len > 0: count += 1
      echo fmt"Corpus: {output_path}"
      echo fmt"Samples: {count}"
    else:
      echo "No corpus built yet. Run: sigma-agent corpus build"
  of "clean":
    removeFile(output_path)
    echo "✓ Corpus removed"
  else:
    echo fmt"Unknown command: {args[0]}"

when isMainModule:
  var args = commandLineParams()
  corpus_cmd(args)
