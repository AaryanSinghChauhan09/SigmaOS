# Security Policy — SigmaOS

## Supported Versions

| Version | Supported |
|---------|-----------|
| `main` (rolling) | ✅ Active |
| Tagged releases | ✅ Latest tag only |
| Older tags | ❌ Not backported |

---

## Reporting a Vulnerability

### Do NOT open a public GitHub issue for security vulnerabilities.

Instead, use one of the following private channels:

### Option A — GitHub Private Security Advisory (preferred)

1. Go to the [Security tab](https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories) of this repo.

2. Click **"Report a vulnerability"**.

3. Fill in the advisory form. We aim to acknowledge within **48 hours**.

### Option B — Email

Send a PGP-encrypted email to: **security@sigmaos.dev** *(placeholder — replace with real address before release)*

```
PGP Key Fingerprint: XXXX XXXX XXXX XXXX XXXX  (add real key before first release)
```

Full public key: `docs/security/pgp-key.asc`

---

## Disclosure Timeline

| Stage | Target |
|-------|--------|
| Initial acknowledgement | 48 hours |
| Triage & severity assessment | 7 days |
| Fix development | 30 days (critical) / 90 days (high) |
| Public disclosure (CVE + advisory) | Upon fix release |

We follow coordinated disclosure — we will not publish details until a fix is available, and we will credit reporters unless they prefer anonymity.

---

## Scope

Security bugs that are **in scope**:

- Kernel privilege escalation (Ring 3 → Ring 0)

- Memory safety violations in `kernel/`, `crypto/`, `security/`

- Capability / Zero-Trust enforcement bypass in `security/`

- Cryptographic implementation errors (especially in `crypto/cryptfs/`)

- Supply-chain attacks against signed release artifacts

### Out of scope (for this project stage):

- Bugs in third-party dependencies (report upstream)

- Theoretical / non-exploitable issues without a PoC

- Social engineering

---

## Severity Rating

We use [CVSS v3.1](https://www.first.org/cvss/) for scoring. Critical (≥9.0) and High (≥7.0) findings are given priority treatment.

---

## Bug Bounty

There is no monetary bounty program at this stage. Reporters of significant findings will be credited in:

- The GitHub Security Advisory

- The `CHANGELOG.md` release notes

- The `CONTRIBUTORS.md` file (if desired)
