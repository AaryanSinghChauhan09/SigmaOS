# SigmaOS Community Governance Model

## Overview

SigmaOS operates under a decentralized, open-source governance charter. Project decisions, package inclusions, and kernel specification changes are managed by the SigmaOS Steering Committee. Decisions are reached through cryptographically signed contributor voting, preventing central-point centralization or hostile takeovers.

### Key Principles

- **Decentralized Governance**: No single point of control
- **Cryptographic Voting**: Signed votes for authenticity
- **Merit-Based**: Contributor status earned through contributions
- **Transparent**: All decisions publicly documented
- **Inclusive**: Open to all contributors
- **Secure**: Cryptographic verification of all votes

## Governance Structure

### Steering Committee

**Composition**:

- 7 elected members
- 12-month term
- Staggered elections (3 seats every 4 months)
- Geographic diversity requirement
- Technical expertise requirement

**Responsibilities**:

- Technical direction decisions
- RFC approval and rejection
- Budget allocation
- Contributor status approval
- Conflict resolution

### Contributor Voting & Onboarding

```text
 [New Contributor] ──► [Submit Valid PR] ──► PR Merged
                                               │
                                               ▼
 [Signed DID Issued] ◄── [Earn Contributor Status] ◄─┘
         │
         ▼
 [Cast Vote on RFCs]
```

### Governance Architecture

```text
┌─────────────────────────────────────────┐
│      SigmaOS Governance               │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Steering │ RFC      │ Contributor  │ │
│  │ Committee│ Process  │ Management   │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      Voting System                     │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Signature│ Vote     │ Tally        │ │
│  │ Verification│ Engine  │ System       │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      DID & Identity                    │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ DID      │ PGP      │ MOK          │ │
│  │ Registry │ Keys     │ Certificates │ │
│  └──────────┴──────────┴──────────────┘ │
└─────────────────────────────────────────┘
```

## Configuration

### Governance Configuration

**File**: `/etc/sigma/governance.conf`

```toml
[steering_committee]
seats = 7
election_term_months = 12
staggered_elections = true
min_geographic_diversity = 3
min_technical_experts = 4

[voting]
quorum_percent = 60
vote_duration_days = 7
require_pgp_signature = true
require_did_signature = true
abstain_counts = false

[contributor]
min_prs_for_status = 5
min_months_active = 3
cla_required = true
code_of_conduct_required = true

[rfc]
min_discussion_days = 7
min_voting_days = 7
merge_threshold_percent = 60
veto_enabled = true
```

## Technical Implementation

### Vote Verification

```rust
// userland/apps/sigma-startup/sigma_startup.nim (simulated voting verify)
use pgp::{PublicKey, Signature};
use did::DID;

pub fn verify_vote_signature(voter_did: &str, vote_hash: &str, signature: &str) -> bool {
    // Verify the signature of a governance vote against the voter's public key
    let public_key = lookup_contributor_key(voter_did);
    verify_signature(public_key, vote_hash, signature)
}

pub fn lookup_contributor_key(voter_did: &str) -> PublicKey {
    // Lookup contributor's public key from DID registry
    let did = DID::parse(voter_did);
    let resolver = DIDResolver::new();
    let doc = resolver.resolve(did).unwrap();

    // Extract public key from DID document
    doc.public_key().clone()
}

pub fn verify_signature(public_key: PublicKey, data: &str, signature: &str) -> bool {
    let sig = Signature::from_base64(signature).unwrap();
    public_key.verify(data.as_bytes(), &sig)
}
```

### RFC Process

```rust
// userland/apps/sigma-governance/src/rfc.rs
pub struct RFCProcess {
    discussion_period: u32,
    voting_period: u32,
    quorum: f32,
}

impl RFCProcess {
    pub fn submit_rfc(&self, rfc: RFC) -> Result<(), GovernanceError> {
        // Validate RFC format
        self.validate_rfc(&rfc)?;

        // Open discussion period
        self.open_discussion(&rfc)?;

        // Wait for discussion period
        tokio::time::sleep(tokio::time::Duration::from_secs(
            self.discussion_period as u64 * 86400
        )).await;

        // Open voting period
        self.open_voting(&rfc)?;

        // Wait for voting period
        tokio::time::sleep(tokio::time::Duration::from_secs(
            self.voting_period as u64 * 86400
        )).await;

        // Tally votes
        let result = self.tally_votes(&rfc)?;

        // Apply decision
        self.apply_decision(&rfc, &result)?;

        Ok(())
    }

    fn tally_votes(&self, rfc: &RFC) -> VoteResult {
        let votes = self.collect_votes(rfc);

        let for_votes = votes.iter().filter(|v| v.vote == VoteType::For).count();
        let against_votes = votes.iter().filter(|v| v.vote == VoteType::Against).count();
        let total_votes = for_votes + against_votes;

        let quorum_met = total_votes >= (self.quorum * self.total_contributors() as f32) as usize;
        let passed = for_votes > against_votes && (for_votes as f32 / total_votes as f32) > 0.6;

        VoteResult {
            for_votes,
            against_votes,
            quorum_met,
            passed,
        }
    }
}
```

### DID Registry

```rust
// userland/apps/sigma-governance/src/did.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DIDDocument {
    pub id: String,
    pub public_key: Vec<PublicKey>,
    pub authentication: Vec<String>,
    pub service: Vec<Service>,
}

#[derive(Serialize, Deserialize)]
pub struct PublicKey {
    pub id: String,
    pub type_: String,
    pub controller: String,
    pub public_key_pem: String,
}

pub struct DIDRegistry {
    documents: HashMap<String, DIDDocument>,
}

impl DIDRegistry {
    pub fn new() -> Self {
        DIDRegistry {
            documents: HashMap::new(),
        }
    }

    pub fn register(&mut self, did: String, document: DIDDocument) -> Result<(), DIDError> {
        // Validate DID format
        self.validate_did(&did)?;

        // Validate document
        self.validate_document(&document)?;

        // Register
        self.documents.insert(did, document);

        Ok(())
    }

    pub fn resolve(&self, did: &str) -> Option<&DIDDocument> {
        self.documents.get(did)
    }
}
```

## Contributor Management

### Contributor Status

**Requirements**:

- 5 merged pull requests
- 3 months of active contribution
- Signed CLA
- Accepted Code of Conduct
- Valid DID

**Benefits**:

- Voting rights on RFCs
- Eligibility for Steering Committee
- Access to contributor-only channels
- Recognition in contributor list

### Contributor License Agreement (CLA)

**CLA Text**:

```text
Contributor License Agreement for SigmaOS

You hereby grant to the SigmaOS project and its successors a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable copyright license to reproduce, prepare derivative works of, publicly display, publicly perform, sublicense, and distribute your contributions and such derivative works.
```

### Code of Conduct

**Key Principles**:

- Respectful communication
- Inclusive environment
- Constructive feedback
- Zero tolerance for harassment
- Conflict resolution process

## RFC Process

### RFC Lifecycle

1. **Draft**: Initial RFC proposal
2. **Discussion**: Community discussion period
3. **Voting**: Contributor voting period
4. **Decision**: Steering committee decision
5. **Implementation**: Implementation phase
6. **Review**: Post-implementation review

### RFC Template

```markdown

# RFC: [Title]

## Status

- [ ] Draft
- [ ] Discussion
- [ ] Voting
- [ ] Accepted
- [ ] Rejected
- [ ] Implemented

## Motivation

Why is this RFC needed?

## Proposed Solution

Detailed description of the proposed solution.

## Alternatives

What alternatives were considered?

## Unresolved Questions

What questions remain unanswered?
```

## Election Process

### Steering Committee Elections

**Eligibility**:

- Contributor status for at least 6 months
- Minimum 10 merged pull requests
- No conflicts of interest
- Geographic diversity consideration

**Process**:

1. Nomination period (7 days)
2. Campaign period (7 days)
3. Voting period (7 days)
4. Vote tallying
5. Results announcement

**Voting System**:

- Single transferable vote (STV)
- Cryptographically signed ballots
- Public verification
- Audit trail

## Best Practices

### Governance

1. **Transparency**: All decisions publicly documented
2. **Inclusivity**: Encourage diverse participation
3. **Merit-Based**: Recognition based on contributions
4. **Security**: Cryptographic verification of all votes

### Contribution

1. **Quality**: Focus on high-quality contributions
2. **Collaboration**: Work with the community
3. **Documentation**: Document all changes
4. **Testing**: Test all contributions thoroughly

### Conflict Resolution

1. **Escalation**: Clear escalation path
2. **Mediation**: Neutral mediation process
3. **Appeals**: Right to appeal decisions
4. **Documentation**: Document all conflicts

## Roadmap & Milestones

### Phase 1 (Months 0-3)

- Establish open-source code of conduct
- Contributor license agreements (CLA)
- Basic governance structure
- RFC process definition

### Phase 2 (Months 3-6)

- GitHub bot for RFC votes
- PGP/MOK signature parsing
- DID registry implementation
- Vote verification system

### Phase 3 (Months 6-9)

- Steering committee election framework
- Dashboard interface
- Contributor management system
- Conflict resolution process

### Phase 4 (Months 9-12)

- Foundation establishment
- Intellectual property protection trust
- Advanced governance features
- Community outreach programs

## References

- [Open Source Governance](https://opensource.guide/governance/)
- [Contributor Covenant](https://www.contributor-covenant.org/)
- [DID Specification](https://www.w3.org/TR/did-core/)
- [PGP Documentation](https://gnupg.org/documentation/)
- [RFC Process](https://rfc.zeromq.org/spec:37/)


---
## Merged from Community-Governance.md
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
