# Community Governance

> SigmaOS is community-driven. Every contributor has a voice.
> Transparent decision-making, open roadmap, and clear contribution paths.

---

## Governance Model

SigmaOS uses a **Benevolent Dictator For Now (BDFN)** model transitioning to **meritocratic governance** as the contributor base grows.

```
Project Lead (BDFN)
       │
       ├── Core Team (kernel, security, AI, packaging)
       │     └── Merge authority for their domain
       │
       ├── Maintainers (per-subsystem ownership)
       │     └── Review + approve PRs in their area
       │
       └── Contributors (everyone else)
             └── PRs, issues, wiki, plugins, packages
```

---

## Decision Making

### Minor decisions (most PRs, bug fixes, docs)

- Any maintainer can approve and merge.

- No RFC required.

- CI must pass.

### Significant decisions (new subsystems, API changes, security)

- RFC (Request For Comments) required: copy `wiki_repo/RFC-Template.md`

- 7-day comment period on the RFC issue

- Core team votes (simple majority)

- Decision documented in wiki

### Strategic decisions (roadmap, governance changes)

- Community RFC with 14-day comment period

- All contributors can vote (1 contributor = 1 vote, based on merged PRs)

- Requires 2/3 supermajority

---

## Contributor Roles

### Contributor

- Anyone who opens a PR, files an issue, or improves the wiki

- No special access required

- Recognized in CONTRIBUTORS file

### Maintainer

- Sustained contribution over 3+ months

- Granted write access to their subsystem

- Responsibilities: review PRs, triage issues, keep CI green

- Nominated by existing maintainers, confirmed by core team

### Core Team Member

- Deep expertise in a critical subsystem

- Merge authority across related areas

- Participates in strategic decisions

- Nominated by project lead or existing core members

### Project Lead

- Final decision authority when consensus fails

- Sets strategic direction

- Manages releases and security disclosures

---

## Contribution Areas

| Area | Skills Needed | Good First Issues |
|---|---|---|
| Kernel | Rust, systems programming | `kernel/` bug fixes |
| AI Agent | Nim, LLM prompting | New tool implementations |
| Package manager | Nim | Package recipes |
| Drivers | Rust, Zig, hardware knowledge | New SDF drivers |
| Documentation | Markdown, SigmaOS knowledge | Wiki improvements |
| Workflows | YAML, automation | New workflow templates |
| Plugins | Nim, shell | New sigma-agent plugins |
| Security | Systems security | Security audit findings |
| Testing | Rust/Nim, testing | New benchmark test cases |
| Translation | Any language + target language | Locale files |

---

## How to Contribute

### 1. Fork and clone

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS
```

### 2. Find something to work on

- Browse [issues labeled `good-first-issue`](https://github.com/AaryanSinghChauhan09/SigmaOS/issues?q=label%3Agood-first-issue)

- Check the [DEVELOPMENT_ROADMAP.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/DEVELOPMENT_ROADMAP.md)

- Ask in Discussions what's needed

### 3. Branch naming convention

```
feature/shard-name-description
fix/module-name-issue-description
docs/wiki-page-name
refactor/subsystem-description
```

### 4. Commit message format

```
type(scope): short description

Longer description if needed.
Closes #123
```
Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `security`

### 5. Submit PR

- CI must pass (12-job pipeline)

- At least 1 maintainer approval

- Update wiki if adding user-facing features

- Include tests for new functionality

---

## Plugin Contributions

Adding new sigma-agent skills doesn't require a core PR:

```bash

# Create a plugin

sigma-agent plugin create my-skill

# Edit ~/.config/sigma/agent/plugins/my-skill/plugin.toml

# Test it

sigma-agent plugin list

# Share it

# Publish to sigma_pkg_registry as sigma-agent-plugin-my-skill

sigma-pkg publish my-skill/
```

Plugin repository: `sigma_pkg_registry/recipes/`

---

## Workflow Template Contributions

```bash

# Create a new workflow template

# 1. Write the YAML

# 2. Add to userland/agent/sigma_agent_workflow.nim WORKFLOW_TEMPLATES array

# 3. Test: sigma-agent workflow install your-template --dry-run

# 4. Submit PR

```

---

## Wiki Contributions

The wiki lives in `wiki_repo/` and is always open for improvements:

```bash

# Clone the wiki

git clone https://github.com/AaryanSinghChauhan09/SigmaOS.wiki.git wiki_repo

# Edit any .md file

# Create new pages for undocumented features

# Commit and push

cd wiki_repo && git add . && git commit -m "docs: improve X" && git push
```

---

## Recognition

Contributors are recognized in:

- `CONTRIBUTORS` file (all code contributors)

- Release notes (features attributed to their authors)

- Wiki maintainers section

- GitHub contributor graph

---

## Code of Conduct

SigmaOS follows the [Contributor Covenant](CODE_OF_CONDUCT). In short:

- Be respectful and constructive

- Harassment of any kind is not tolerated

- Focus on the work, not the person

- Disagreement is fine; personal attacks are not

Report issues to: conduct@sigmaos.dev (or open a private GitHub issue)

---

## Roadmap Voting

Every quarter, contributors can vote on the next quarter's priorities:

1. Core team proposes a list of features/improvements

2. All contributors with ≥1 merged PR get 3 votes each

3. Results published as the quarterly roadmap

4. Tracked in DEVELOPMENT_ROADMAP.md

---

*See also: [Contributing](CONTRIBUTING) · [Developer Guide](Developer_Guide) · [SDK Guide](SDK-Guide)*
