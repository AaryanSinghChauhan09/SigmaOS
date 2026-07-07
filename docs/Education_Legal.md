# SigmaOS Education & Legal Compliance Suite

## Overview
SigmaOS incorporates a complete educational and legal workstation suite. This includes native engineering/math tools (GeoGebra, Scilab, Octave), institutional administration systems (Moodle, ERPNext, Koha, GNUCash, QGIS), and a localized legal research module integrating secure API workflows for the Indian Kanoon database and SCC Online.

## Architecture & API Workflows
The legal research module allows attorneys and civil servants to query judgments, case law, and regulations offline or via secure API integrations.

```
 [User Search Query] ──► [Local SQLite Cache] ──► Found?
                                │                  │
                        No ◄────┘                  ├──► Yes ──► Render Document
                        │                          ▼
                        ▼                     [Verify DID Signature]
            [Indian Kanoon API Query]
                        │
                        ▼
             [Format Judgments (JSON)]
                        │
                        ▼
             [Cache to local storage]
```

## System Properties
API parameters and offline indices are configured in `/etc/sigma/legal.conf`:
```toml
[apis.indian_kanoon]
api_url = "https://api.indiankanoon.org/search/"
access_token_env = "INDIANKANOON_TOKEN"
cache_expiry_days = 30

[compliance]
default_jurisdiction = "IN"
audit_logging = true
```

## Technical Implementation
The legal engine is implemented in low-level Nim/Rust to ensure memory safety and zero dependency on heavy node/python runtimes.

```rust
// userland/apps/sigma-legal/src/lib.rs
pub struct LegalEngine {
    pub client: reqwest::blocking::Client,
    pub api_token: String,
}

impl LegalEngine {
    pub fn query_statute(&self, act_name: &str) -> Result<String, LegalError> {
        let url = format!("https://api.indiankanoon.org/act/{}", act_name);
        let resp = self.client.get(&url)
            .header("Authorization", format!("Token {}", self.api_token))
            .send()?
            .text()?;
        Ok(resp)
    }
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Base packages compiled (GeoGebra, Scilab, Octave, QGIS).
- **Phase 2 (Months 3-6)**: SQL database cache and schema design for Indian Kanoon offline statute indices.
- **Phase 3 (Months 6-9)**: Case management application for court filing automation and compliance tracking.
- **Phase 4 (Months 9-12)**: SCC Online enterprise integration with custom single-sign-on (SSO) gateways.
