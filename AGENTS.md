# SigmaOS — Agent & Contributor Rules

SigmaOS is a sovereign, modular operating system. This repo (`SigmaOS`) is the
**connector/control plane** for the `SigmaOS-*` component fleet — it owns the
registry (`COMPONENTS.toml`), the canonical scaffold (`templates/component/`),
and the shell hub (`scripts/sigma_hub.sh`). Each component lives in its own
`SigmaOS-<name>` repository (Omarchy-style multi-repo layout).

## Prime directives

- **NO DEMO & NO STUB — only actual working code.** Scaffold repos must be
  upgraded to real, tested implementations; never leave placeholder logic.
- **TALK IS CHEAP, SHOW ME THE CODE.** Every change must build and pass tests.
- **TEST CODE USING CI/CD.** Run `cargo test` / `make hub-test`; AAA pattern.
- **ONE COMPONENT PER REPO; ONE `main` BRANCH PER REPO.**
- **CONSOLIDATE similar repos; CREATE only necessary components.**

## Engineering principles (apply always)

Modularity · single responsibility · DRY · KISS · separation of concerns ·
mechanism vs policy · information hiding · loose coupling · high cohesion ·
interface segregation · dependency inversion · open/closed · Liskov ·
composition over inheritance · fail-fast · least privilege · secure defaults ·
defensive programming · atomic ops · race-free design · deadlock avoidance ·
zero-copy where possible · reproducible builds · semantic versioning ·
upstream-first · pristine sources · optimize for deletion.

## Component contract

Every `SigmaOS-*` repo carries `COMPONENT.toml` (`[component]`, `[contract]
provides/depends_on`, `[maturity]`). The registry `COMPONENTS.toml` is the
source of truth — keep metadata truthful (no invented descriptions).

## Hub workflow (`scripts/sigma_hub.sh`)

Use the hub for all fleet ops — never hand-edit across repos blindly:
`list status audit families consolidate dupes health` (inventory) ·
`search info depends` (package-query, Arch/Debian/Fedora/Mint/Omarchy/Ubuntu-
inspired) · `connect create upgrade sync push test` (lifecycle).
Make targets: `make hub-*`. Prefer `--dry-run` first; `upgrade`/`create` are
non-destructive (add missing files only).

## Safety

- Never commit secrets/keys/tokens — env vars or a secret manager only.
- Consolidation/archival of remote repos is **reversible-preferred** (archive,
  don't delete) and requires verification each group isn't unique work.
- Branch only for features needing tests; merge after tests pass.
- Keep `main` free of `.md`/`.txt` docs — those belong in the GitHub Wiki.

## Verification before "done"

`cargo build --offline` + `cargo test --offline` green; `COMPONENT.toml` valid
TOML; changes committed; push only when asked or via `make hub-sync --push`.
</content>
</invoke>

Wait — I need to actually write the file. Let me use the write tool properly.

Actually I made an error — I output the content as text instead of calling the tool. Let me write it properly.<|close|>think<|sep|><|open|>response<|sep|>Let me write that properly.<|close|>response<|sep|><|open|>tools<|sep|><|open|>call tool="write" index="1"<|sep|><|open|>argument key="file_path" type="string"<|sep|>/home/aaryansinghchauhan/SigmaOS/AGENTS.md