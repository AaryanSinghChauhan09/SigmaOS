# SigmaOS — Indian Profession Tools Development Roadmap (Per Branch)

Detailed, file-level development plan for all 55+ India profession apps
across every branch. Grounded in June 2026 codebase state.

---

## Implementation State (Ground Truth)

### 55 profession apps in `userland/apps/`.

| State | Count | Apps |
|-------|-------|------|
| `.cpp` implemented | 7 | sigma-agri ✅, sigma-edu, sigma-gov, sigma-labour, sigma-bank, sigma-realty, sigma-startup |
| `.h` only (API defined, no bodies) | 48 | All others |
| Missing entirely | 0 | All 55 have at least a header |

**sigma-agri is the most complete** — real MSP table (26 crops × FY2025-26), PMFBY premium calculator, eNAM registration flow, PM-Kisan status, soil health card. No live API calls yet.

**sigma-accounts has the richest header** — full double-entry engine, GST vouchers, e-Invoice IRN, eWay Bill, Tally import/export, DID-signed audit trail. Bodies missing.

---

## What Every App Needs (Standard Completion Template)

For each app, moving from `.h` to production requires:

```

1. Core business logic (.cpp body)      ← implement API functions

2. CLI entry point (sigma-<app> main)   ← command-line interface

3. India Stack API client               ← live government API integration

4. Offline data bundle                  ← work without internet

5. DID signature integration            ← tamper-proof audit trail

6. sigma-bus IPC registration           ← OS-level integration

7. sigma-pkg recipe                     ← installable as .spkg

8. Tests + CI                           ← regression prevention
```

---

## `tools-dev` — Core Profession App Infrastructure

Everything profession apps depend on. Build this first.

### India Stack API client layer

**New file:** `userland/indiastack/sigma_indiastack_client.cpp`

Existing: `userland/indiastack/sigma_indiastack.h` — header only.

| Task | File | Detail |
|------|------|--------|
| ABDM OAuth2 token flow | `userland/indiastack/sigma_abdm_client.cpp` | ABHA creation, PHR linking, FHIR R4 push/pull |
| GSTN API client (IRP) | `userland/indiastack/sigma_gstn_client.cpp` | IRN generation, e-Way Bill, GSTR filing via NIC IRP |
| UPI API client (NPCI) | `userland/indiastack/sigma_upi_client.cpp` | Pay/collect, mandate, e-RUPI |
| DigiLocker API | `userland/indiastack/sigma_digilocker_client.cpp` | Fetch/push/verify government documents |
| Aadhaar eKYC client | `userland/indiastack/sigma_aadhaar_client.cpp` | Offline XML + OTP-based eKYC |
| NIC APIs (common auth) | `userland/indiastack/sigma_nic_client.cpp` | NIC SSO, MCA21, IndiEA services |
| NavIC location client | `userland/indiastack/sigma_navic_client.cpp` | ISRO NavIC receiver serial/USB |
| Bhashini API client | `userland/indiastack/sigma_bhashini_client.cpp` | ASR/TTS/NMT for 22 Indian languages |
| Account Aggregator (AA) | `userland/indiastack/sigma_aa_client.cpp` | FIP/FIU consent, financial data fetch |
| India Stack offline cache | `userland/indiastack/sigma_indiastack_cache.cpp` | SQLite cache of HSN/SAC, ICD-10, court codes |

### sigma-bus profession service registry

**New file:** `userland/indiastack/sigma_profession_bus.cpp`

```cpp
// Every profession app registers on sigma-bus:
sigma_bus_register("sigma-ca",      SIGMA_BUS_PROFESSION_CA);
sigma_bus_register("sigma-health",  SIGMA_BUS_PROFESSION_DOCTOR);
sigma_bus_register("sigma-accounts",SIGMA_BUS_PROFESSION_ACCOUNTS);
// Cross-app: invoice from sigma-accounts auto-posts to sigma-ca dashboard
```

### sigma-DID professional credential

**File:** `security/SovereignDID.cpp` (extend existing)

```bash
sigma-sec did professional-credential add \
  --body ICAI --license-number 123456

# Generates ML-DSA-87 signed QR code

# Accepted by: government portals, court e-filing, ABDM

```

---

## `release/standalone` — Full Desktop Profession Apps

### Priority 1: Financial Tools (sigma-ca, sigma-accounts, sigma-cs)

#### sigma-ca (Chartered Accountant)

**File:** `userland/apps/sigma-ca/sigma_ca.h` → add `sigma_ca.cpp`

```
sigma-ca dashboard                    # multi-client overview

sigma-ca client add <name> <gstin>    # add client

sigma-ca client list
sigma-ca gst compute <gstin> <period> # compute GSTR amounts

sigma-ca gst file <gstin> <period>    # file GSTR-1/3B/9/9C via GSTN API

sigma-ca gst reconcile <gstin> <period>  # 2A/2B vs purchase register

sigma-ca einvoice generate <json>     # generate IRN + QR code

sigma-ca itr compute <pan> <ay>       # compute income tax liability

sigma-ca itr fetch-26as <pan> <ay>    # fetch Form 26AS from TRACES

sigma-ca tds compute <form> <amount> <section>
sigma-ca tds file <quarter>           # file 24Q/26Q/27Q

sigma-ca capital-gains compute <asset>
sigma-ca audit plan <client> <period> # Ind AS compliance checklist

sigma-ca form-16 generate <pan>       # generate Form 16 PDF

sigma-ca balance-sheet <client> <fy>  # P&L + balance sheet

```

| Task | File | API |
|------|------|-----|
| `sigma_gst_compute()` body | `sigma_ca.cpp` | GSTN sandbox API |
| `sigma_gst_file()` body | `sigma_ca.cpp` | GSTN production IRP |
| `sigma_einvoice_generate()` body | `sigma_ca.cpp` | NIC IRP `/einvoice/auth` |
| `sigma_itr_compute()` body | `sigma_ca.cpp` | offline tax slabs FY2025-26 |
| `sigma_itr_fetch_26as()` body | `sigma_ca.cpp` | TRACES API |
| `sigma_tds_calculate()` body | `sigma_ca.cpp` | TDS rate chart offline |

#### sigma-accounts (Business Accounting)

**File:** `userland/apps/sigma-accounts/sigma_accounts.h` → add `sigma_accounts.cpp`

```
sigma-accounts init <company> <gstin>  # create company ledger

sigma-accounts voucher sales <json>    # post sales invoice

sigma-accounts voucher purchase <json> # post purchase invoice

sigma-accounts voucher payment <json>  # post payment

sigma-accounts voucher list [--date <period>]
sigma-accounts balance <account>
sigma-accounts trial-balance [--period <fy>]
sigma-accounts profit-loss [--period <fy>]
sigma-accounts balance-sheet [--period <fy>]
sigma-accounts gstr1 <period>          # auto-populate from vouchers

sigma-accounts gstr3b <period>
sigma-accounts irn generate <invoice-id>   # e-Invoice IRN

sigma-accounts eway generate <invoice-id>  # eWay Bill

sigma-accounts import tally <xml>      # import Tally XML

sigma-accounts export tally <xml>      # export Tally-compatible

sigma-accounts audit verify            # verify all DID signatures

```

| Task | File | Detail |
|------|------|--------|
| `sigma_accounts_post()` body | `sigma_accounts.cpp` | Validate double-entry, write SQLite |
| `sigma_accounts_gstr1()` body | `sigma_accounts.cpp` | Aggregate invoices → GSTR-1 JSON |
| `sigma_accounts_generate_irn()` body | `sigma_accounts.cpp` | HTTP POST to GSTN IRP |
| `sigma_accounts_import_tally()` body | `sigma_accounts.cpp` | Parse Tally XML schema |
| SQLite ledger schema | `sigma_accounts.cpp` | Create tables: accounts, vouchers, entries |
| DID signature on every voucher | `sigma_accounts.cpp` | Call `pqc_sign()` on voucher hash |

#### sigma-cs (Company Secretary)

**File:** `userland/apps/sigma-cs/sigma_cs.h` → add `sigma_cs.cpp`

```
sigma-cs mgt7 <cin> <fy>             # Annual return (MGT-7)

sigma-cs aoc4 <cin> <fy>             # Financial statements (AOC-4)

sigma-cs board-meeting create <date> # SS-1 compliant notice

sigma-cs board-meeting minutes <id>  # record minutes

sigma-cs egm create <type> <date>    # Extraordinary General Meeting

sigma-cs share-transfer <from> <to> <shares>
sigma-cs dir-8 <din> <company>       # Director disclosure

sigma-cs statutory-register list     # MGT-1/PAS-3/SH-7

sigma-cs mca-filing <form> <cin>     # file on MCA21 portal

```

### Priority 2: Health Tools (sigma-health, sigma-pharma, sigma-dental)

#### sigma-health (Doctor / ABDM)

**New file:** `userland/apps/sigma-health/sigma_health.cpp`

```
sigma-health patient create <name> <abha>  # create patient record

sigma-health patient search <name|abha>
sigma-health emr view <patient-id>         # view EMR

sigma-health emr update <patient-id>       # add SOAP notes

sigma-health prescribe <patient-id>        # generate NMC e-prescription

sigma-health prescribe verify <rx-id>      # verify Rx DID signature

sigma-health lab order <patient-id> <test> # order lab tests

sigma-health lab result <patient-id>       # fetch FHIR DiagnosticReport

sigma-health pmjay claim <patient-id>      # submit PMJAY claim (NHCX)

sigma-health abdm consent <patient-id>     # ABDM consent management

sigma-health drug check <drug1> <drug2>    # drug interaction check

sigma-health icd10 search <term>           # ICD-10 code lookup

```

| Task | File | API |
|------|------|-----|
| ABDM FHIR R4 client | `sigma_health.cpp` | ABDM Sandbox → Production |
| NMC e-prescription format | `sigma_health.cpp` | Offline NMC template |
| Drug interaction DB | `sigma_health.cpp` | SQLite offline (WHO AEDS) |
| PMJAY claim NHCX | `sigma_health.cpp` | NHCX API `POST /claim` |
| ICD-10 offline DB | `sigma_health.cpp` | SQLite, 12,000+ codes |

#### sigma-pharma (Pharmacist)

**File:** `userland/apps/sigma-pharma/sigma_pharma.h` → add body

```
sigma-pharma stock add <drug> <batch> <expiry> <qty>
sigma-pharma stock list [--expiry-before <date>]
sigma-pharma dispense <rx-id> <patient>      # verify Rx, update stock

sigma-pharma schedule-h log <drug> <qty>     # Schedule H/X narcotic log

sigma-pharma ndps register <entry>           # NDPS register entry

sigma-pharma expiry alert                    # drugs expiring in 30 days

sigma-pharma bill <patient-id>               # generate GST invoice

sigma-pharma cdsco check <drug>              # CDSCO drug approval status

```

### Priority 3: Legal Tools (sigma-legal, sigma-police)

#### sigma-legal (Advocate)

**File:** `userland/apps/sigma-legal/sigma_legal.h` → add body

```
sigma-legal case create <client> <court> <type>
sigma-legal case list [--status pending]
sigma-legal hearing add <case-id> <date> <court>
sigma-legal brief draft <case-id>            # DID-signed brief

sigma-legal ecourt status <cnr>              # eCourts API case status

sigma-legal cause-list <court-code> <date>   # day's cause list

sigma-legal ipc-bns map <section>            # IPC → BNS 2023 mapper

sigma-legal kalyanasundaram lookup <act>     # bare act lookup

sigma-legal fee invoice <client> <amount>    # legal fee invoice (GST)

sigma-legal limitation check <date> <act>    # limitation period calculator

sigma-legal stamp-duty <state> <doc-type> <value>
```

#### sigma-police (Law Enforcement)

**File:** `userland/apps/sigma-police/sigma_police.h` → add body

```
sigma-police fir draft <complainant> <sections>  # FIR under BNSS

sigma-police fir number <station> <year>         # next FIR number

sigma-police ipc-to-bns <section>                # IPC→BNS/BNSS 2023 map

sigma-police challan traffic <vehicle> <section> # e-Challan under MV Act

sigma-police cctns lookup <name> <dob>           # CCTNS record check

sigma-police arrested-person rights              # BNSS Sec 36 rights card

sigma-police witness statement <case-id>
```

---

### Priority 4: Agriculture (sigma-agri — extend existing .cpp)

**File:** `userland/apps/sigma-agri/sigma_agri.cpp` — most complete app.

```
sigma-agri msp --crop wheat --year 2026     # [✅ real — 26 crops]

sigma-agri msp --list                       # [✅ real — table]

sigma-agri insurance premium --crop paddy --state PB --area 2.5  # [✅ real]

sigma-agri insurance enroll --crop paddy --state PB  # [❌ PMFBY API]

sigma-agri weather --district Ludhiana --forecast 7  # [⚠️ offline stub]

sigma-agri enam register --fpo --district Amritsar    # [⚠️ stub]

sigma-agri pmkisan status --aadhar-last4 1234         # [⚠️ stub]

sigma-agri soil --plot-id P001 --nutrient NPK         # [⚠️ stub]

sigma-agri kcc apply --bank SBI --limit 50000         # [❌ KCC API]

sigma-agri enam prices --commodity wheat --mandi Azadpur  # [❌ eNAM API]

sigma-agri land records --state UP --khatauni 12345   # [❌ DILRMP API]

sigma-agri subsidy list --state MP                    # [❌ DBT API]

sigma-agri crop calendar --district Pune --crop sugarcane  # [❌ build]

sigma-agri drone permit --village <id>                # [❌ DGCA API]

sigma-agri fpo register <name> <district>             # [❌ SFAC API]

```

| Task | File | API |
|------|------|-----|
| eNAM live prices | `sigma_agri.cpp` | enam.gov.in REST API |
| PMFBY enrollment | `sigma_agri.cpp` | pmfby.gov.in enroll API |
| Land records (DILRMP) | `sigma_agri.cpp` | DILRMP state land APIs |
| IMD weather live | `sigma_agri.cpp` | api.weather.imd.gov.in |
| KCC bank API | `sigma_agri.cpp` | Jan Samarth portal |
| DBT subsidy status | `sigma_agri.cpp` | DBT Bharat API |

### Priority 5: Education (sigma-edu — extend existing .cpp)

**File:** `userland/apps/sigma-edu/sigma_edu.cpp`

```
sigma-edu attendance mark <class> <date>       # daily attendance

sigma-edu attendance report <class> <month>
sigma-edu marks add <student> <subject> <score>
sigma-edu report-card generate <student> <term>
sigma-edu udise report <school-code>           # UDISE+ school data

sigma-edu rte admit <student>                  # RTE 25% quota admission

sigma-edu mid-day-meal log <date> <count>      # PM Poshan attendance

sigma-edu digilocker certificate <student-id>  # push marks to DigiLocker

sigma-edu salary slip <teacher-id> <month>     # EPFO-compliant pay slip

sigma-edu pta meeting schedule <date>
```

### Priority 6: Government Employee (sigma-gov — extend existing .cpp)

**File:** `userland/apps/sigma-gov/sigma_gov.cpp`

```
sigma-gov pfms voucher <amount> <purpose>      # PFMS payment

sigma-gov gem order <category> <budget>        # GeM marketplace order

sigma-gov sparrow report <officer> <period>    # SPARROW ACR entry

sigma-gov travel lta <year>                    # LTA claim

sigma-gov gpf statement <empid>                # GPF account statement

sigma-gov service book <empid>                 # service book entry

sigma-gov nps statement <pran>                 # NPS account balance

sigma-gov attendance biometric <empid>         # AEBAS biometric

sigma-gov rti file <authority> <subject>       # RTI application

sigma-gov cpgrams complaint <dept> <text>      # CPGRAMS grievance

```

---

## `release/cloud` — Profession Apps as Hosted Services

Cloud profile = profession apps run as sigma-pod containers, no GUI.

### sigma-ca as API service

```bash

# Deploy CA dashboard as cloud service

sigma-pod run-native sigma-ca.spkg \
  --all-ns --cpu=500 --mem=256 \
  --env GSTN_CLIENT_ID=xxx \
  --env GSTN_SECRET=yyy \
  --port 8443

# API endpoints available to fleet devices

sigma-fleet push-policy --profile sigma-ca-cloud
```

| Task | File | Detail |
|------|------|--------|
| HTTP API wrapper for sigma-ca | `userland/apps/sigma-ca/sigma_ca_api.cpp` | gRPC or REST `/gst/file`, `/itr/compute` |
| Multi-tenant client isolation | `userland/apps/sigma-ca/sigma_ca_api.cpp` | Per-GSTIN sigma-pod container |
| Audit log to SovereignCloudFS | `userland/apps/sigma-ca/sigma_ca_api.cpp` | All filings replicated across nodes |

### sigma-health as telemedicine service

```bash
sigma-pod run-native sigma-health.spkg \
  --cpu=1000 --mem=512 \
  --env ABDM_CLIENT_ID=xxx
```

| Task | File | Detail |
|------|------|--------|
| ABDM FHIR server endpoint | `userland/apps/sigma-health/sigma_health_api.cpp` | `POST /Patient`, `POST /Bundle` |
| Video consultation hook | `userland/apps/sigma-health/sigma_health_api.cpp` | sigma-display Vulkan video stream |
| Prescription API for hospitals | `userland/apps/sigma-health/sigma_health_api.cpp` | `POST /MedicationRequest` (FHIR) |

### sigma-gram as panchayat cloud node

```bash
sigma-pod run-native sigma-gram.spkg \
  --cpu=250 --mem=128 \
  --env MGNREGS_STATE=MP
```

| Task | File | Detail |
|------|------|--------|
| MGNREGS job card API | `userland/apps/sigma-gram/sigma_gram.cpp` | NREGASoft API integration |
| Birth/death registration | `userland/apps/sigma-gram/sigma_gram.cpp` | CRVSUP API via MoHFW |
| JJM water supply status | `userland/apps/sigma-gram/sigma_gram.cpp` | Jal Jeevan Mission dashboard API |
| e-GramSwaraj integration | `userland/apps/sigma-gram/sigma_gram.cpp` | MoPR e-GramSwaraj API |

---

## `release/mobile` — Profession Apps on ARM64 / sigma-ultra

### sigma-ultra-lite (feature phone, 16 MB RAM)

**File:** `userland/apps/sigma-ultra-lite/sigma_ultra_lite.h` → add body

Targets: JioPhone, basic Android with sigma-ultra, USSD fallback.

```
sigma-ultra msp             # MSP check via USSD menu

sigma-ultra pmkisan         # PM-Kisan status

sigma-ultra balance         # UPI/IPPB balance

sigma-ultra pay <vpa> <amt> # UPI payment

sigma-ultra weather         # offline district weather

sigma-ultra ration          # PDS ration card status

sigma-ultra mgnregs         # MGNREGS attendance

sigma-ultra health          # nearest PHC + ambulance 108

```

| Task | File | Detail |
|------|------|--------|
| USSD menu engine | `sigma_ultra_lite.cpp` | 160-char text, `*999#` trigger |
| Offline MSP table embed | `sigma_ultra_lite.cpp` | Include `msp_table[]` from sigma-agri |
| UPI via USSD (`*99#`) | `sigma_ultra_lite.cpp` | NPCI NUUP USSD protocol |
| 2G packet compression | `sigma_ultra_lite.cpp` | LZ4 compress API responses |

### ARM64-optimised profession tools

| App | ARM64 task | File | Detail |
|-----|-----------|------|--------|
| sigma-agri | NEON-accelerated NDVI calc | `sigma_agri.cpp` | Float32 NEON intrinsics for satellite image processing |
| sigma-health | Camera → FHIR image | `sigma_health.cpp` | RPi camera → JPEG → ABDM DocumentReference |
| sigma-pos | NFC UPI tap-to-pay | `sigma_pos.cpp` | HCE (Host Card Emulation) via NFC HAL |
| sigma-edu | Offline speech-to-text | `sigma_edu.cpp` | sigma-bhashini ASR on ARM via NEON |
| sigma-gram | NavIC GPS field map | `sigma_gram.cpp` | NavIC serial → lat/lon → boundary polygon |

---

## `release/distributed` — Federated Profession Data

### sigma-fedlearn profession networks

```bash

# Federated learning — no raw data leaves device

sigma-fl-coordinator start \
  --network sigma-tax-anomaly \
  --participants 100 \
  --rounds 10 \
  --privacy epsilon=1.0

# CAs training GST anomaly detector:

sigma-ca fedlearn train --network sigma-tax-anomaly

# Farmers training crop disease model:

sigma-agri fedlearn train --network sigma-crop-disease

# Doctors training triage model (no patient data shared):

sigma-health fedlearn train --network sigma-triage
```

| Task | File | Detail |
|------|------|--------|
| sigma-tax-anomaly FL network | `userland/apps/sigma-ca/sigma_ca_fedlearn.cpp` | Gradient aggregation, Dilithium3-signed updates |
| sigma-crop-disease FL network | `userland/apps/sigma-agri/sigma_agri_fedlearn.cpp` | Image classification, federated averaging |
| sigma-triage FL network | `userland/apps/sigma-health/sigma_health_fedlearn.cpp` | Symptom→triage model, DPDP-compliant |

### sigma-blockchain-lite for profession records

```bash
sigma-blockchain record add \
  --type land-registry \
  --state TN --district Chennai \
  --data "deed_hash=abc123"

sigma-blockchain verify --id tx-001
```

Used by: sigma-realty (land), sigma-legal (judgments), sigma-gov (tenders)

---

## `release/rtos` — Real-Time Profession Apps

### Industrial & safety applications requiring hard RT

| App | RT requirement | Task |
|-----|---------------|------|
| sigma-mining | Accident report within 2 hours (DGMS) | RTOS alert pipeline: sensor → sigma-safety → DGMS API within 2 hr |
| sigma-petroleum | Level sensor polling < 100 ms | EDF task: read dip sensor every 100 ms |
| sigma-aviation | METAR/TAF refresh < 30 s | RT daemon: poll MET server, alert on SIGMET |
| sigma-electrical | PMT relay trip < 50 ms | EDF task: monitor CT/PT, trip relay |
| sigma-safety | Emergency evacuation alarm | Highest-priority EDF task, < 10 ms to activate alarm |
| sigma-aerb | Dose rate alarm < 1 s | Geiger counter polling, AERB report |

```bash

# Real-time mining safety daemon

sigma-rt set $(sigma-pod inspect sigma-mining | jq .pid) \
  --policy edf \
  --deadline 1000000  # 1 ms deadline

  --period 100000000  # 100 ms period

```

---

## `release/microkernel` — Minimal Profession Tools

Only the smallest, most critical profession functions ship.
Target: government field offices, BharatNet nodes, basic kiosks.

### Minimal set (fits in 64 MB RAM)

```
sigma-agri msp --crop wheat       # offline MSP lookup

sigma-agri pmkisan status         # PM-Kisan check

sigma-gram mgnregs attendance     # MGNREGS job card

sigma-pos bill <amount>           # basic UPI billing

sigma-health emergency contacts   # 108/112 quick dial

sigma-sec did verify <qr>         # verify DID credential

sigma-pqc verify <sig> <pk>       # verify document signature

```

| Task | File | Detail |
|------|------|--------|
| Bundle offline lookup tables | Makefile | MSP, ICD-10-basic, HSN top-200 in rodata |
| Strip all GUI code | Build flags | `-DSIGMA_MICROKERNEL_PROFILE` exclude Zenith |
| Compress static data | Makefile | LZ4 compress embedded lookup tables |

---

## `release/dual-boot` — Profession Data Migration

When users install SigmaOS alongside Windows/Linux, existing profession
data must migrate cleanly.

```bash
sigma-install migrate --from tally --to sigma-accounts
sigma-install migrate --from winEHR --to sigma-health
sigma-install migrate --from excel-accounts --to sigma-accounts
```

| Task | File | Detail |
|------|------|--------|
| Tally XML → sigma-accounts | `userland/installer/sigma_migrate_tally.cpp` | Call `sigma_accounts_import_tally()` |
| Excel ledger → sigma-accounts | `userland/installer/sigma_migrate_excel.cpp` | Parse .xlsx (libxlsxwriter) |
| Windows EHR export → ABDM FHIR | `userland/installer/sigma_migrate_ehr.cpp` | Map HL7 v2 → FHIR R4 |
| Browser bookmarks → sigma-legal | `userland/installer/sigma_migrate_legal.cpp` | Extract court case URLs → case database |

---

## `release/browser` — Profession Web Demos

```bash

# Browser-hosted demos (sigma-web, WebAssembly):

sigma-web open https://sigmaos.dev/demo/sigma-ca
sigma-web open https://sigmaos.dev/demo/sigma-agri
sigma-web open https://sigmaos.dev/demo/sigma-health
```

| Task | File | Detail |
|------|------|--------|
| sigma-ca WASM build | `userland/apps/sigma-ca/Makefile` | `emcc sigma_ca.cpp -o sigma_ca.wasm` |
| sigma-agri WASM build | `userland/apps/sigma-agri/Makefile` | MSP lookup + premium calc in browser |
| sigma-accounts WASM build | `userland/apps/sigma-accounts/Makefile` | Demo voucher entry in browser |
| Demo landing pages | `browser/demos/` | HTML + JS wrappers for each WASM |

---

## `release/app` — App Store Profession Listings

```bash
sigma-app search doctor

# → sigma-health v1.2 — ABDM EMR, PMJAY claims, NMC e-prescription

sigma-app install sigma-ca

# → Downloads sigma-ca.spkg, verifies Dilithium3, installs

sigma-app info sigma-agri

# → Shows: crops covered, APIs used, offline capability, permissions needed

```

| Task | File | Detail |
|------|------|--------|
| App store cards for all 55 apps | `app_store.html` | Icon, description, permissions, install button |
| sigma-pkg recipes for all apps | `sigma_pkg_registry/recipes/` | One `.recipe` file per profession app |
| Dilithium3-sign all .spkg files | `scripts/sign_release.sh` | Sign during release pipeline |
| Permission manifests | `userland/apps/*/manifest.sigma` | Declare required capabilities |

---

## `kernel-exp` — Kernel Support for Profession Apps

### What the kernel must provide for profession apps to work

| Requirement | App(s) | Kernel task | File |
|------------|--------|-------------|------|
| VFS file read/write | All apps | VFS bodies | `kernel/vfs/sigma_vfs.cpp` |
| SQLite via sigma-vfs | sigma-accounts, sigma-health, sigma-agri | VFS + mmap | `kernel/vfs/sigma_vfs.cpp` |
| Network socket API | All India Stack APIs | TCP stack | `net/sigma_net_tcp.cpp` |
| sigma-bus IPC | Cross-app integration | sigma-bus bodies | `kernel/ipc/sigma_bus.cpp` |
| Timer (INR deadline reminders) | sigma-ca, sigma-accounts | APIC timer | `kernel/core/sigma_timer.cpp` |
| TLS for API calls | All India Stack clients | sigma-tls | `net/tls/sigma_tls.cpp` |
| DID signature via kernel | All audit trail | sigma-trustd | `security/SovereignDID.cpp` |
| cgroup isolation per app | Cloud profession services | cgroup v2 | `kernel/core/process/sigma_cgroup.c` |

### sigma-bus topic map for profession apps

```
TOPIC                          PUBLISHER          SUBSCRIBER
sigma.gst.invoice.posted    ← sigma-accounts  → sigma-ca, sigma-hrms
sigma.health.prescription   ← sigma-health    → sigma-pharma
sigma.agri.msp.updated      ← sigma-agri      → sigma-pos (mandi terminal)
sigma.legal.hearing.due     ← sigma-legal     → sigma-cli (notification)
sigma.payroll.salary.posted ← sigma-hrms      → sigma-ca (TDS calc)
sigma.fleet.location        ← sigma-transport → sigma-accounts (billing)
```

---

## `fs-dev` — Data Storage for Profession Apps

Every profession app stores data in SigmaFS. `fs-dev` must provide:

| App | Storage need | fs-dev task |
|-----|-------------|------------|
| sigma-accounts | SQLite ledger, Tally XML cache | VFS mmap + tmpfs staging |
| sigma-health | FHIR JSON bundles, medical images | Large file support + dm-verity |
| sigma-ca | 7-year GSTN return archive | SigmaFS compression + encryption |
| sigma-agri | Satellite images, sensor logs | Large file + UBC read-ahead |
| sigma-legal | Case documents, briefs, orders | Full-text search via SQLite FTS5 |
| sigma-edu | Student records per school year | Per-FY partition rotation |
| sigma-gram | Land maps, survey data | GeoJSON in SQLite + spatial index |

```bash

# Encrypted profession data partition

sigma-fs mkfs /dev/nvme0n1p3 --type sigmafs --encrypted --label sigma-data
sigma-fs verity enable /dev/nvme0n1p3
sigma-fs mount /dev/nvme0n1p3 /sigma/data/profession
```

---

## `drivers-dev` — Hardware for Profession Apps

| App | Hardware requirement | Driver task |
|-----|---------------------|-------------|
| sigma-health | Camera (Raspberry Pi / USB webcam) | `drivers/camera/sigma_v4l2.cpp` |
| sigma-pos | NFC reader (UPI tap-to-pay) | `drivers/nfc/sigma_nfc.cpp` |
| sigma-pos | Thermal receipt printer | `drivers/printer/sigma_thermal.cpp` |
| sigma-agri | NavIC GPS receiver (serial/USB) | `drivers/serial/sigma_navic.cpp` |
| sigma-dental | X-ray sensor (USB DICOM) | `drivers/usb/sigma_dicom.cpp` |
| sigma-electrical | USB energy meter (Modbus RTU) | `drivers/serial/sigma_modbus.cpp` |
| sigma-mining | Gas sensor (I2C, Raspberry Pi) | `drivers/i2c/sigma_gas_sensor.cpp` |
| sigma-pharma | Barcode scanner (USB HID) | `drivers/usb/sigma_barcode.cpp` |
| sigma-gram | Iris scanner (UIDAI eKYC) | `drivers/biometric/sigma_iris.cpp` |
| sigma-aerb | Geiger counter (serial) | `drivers/serial/sigma_geiger.cpp` |

---

## `performance-optimized` — Profession App Performance

| App | Performance target | Task |
|-----|-------------------|------|
| sigma-accounts | GST computation < 50 ms for 10,000 invoices | SIMD-vectorised summation in AVX-512 |
| sigma-ca | GSTR-1 JSON generation < 100 ms | Memory-mapped voucher table, lock-free |
| sigma-agri | NDVI satellite image process < 2 s | NEON/AVX-512 float32 vectorisation |
| sigma-health | Drug interaction check < 10 ms | BTree index on SQLite drug DB |
| sigma-legal | Full-text case search < 200 ms | SQLite FTS5 with rank() |
| sigma-pos | UPI transaction confirm < 1 s | Async sigma-bus IPC, non-blocking TLS |
| sigma-pqc | Sign GST invoice with ML-DSA < 5 ms | Real Dilithium NTT (FIPS 204) |

---

## `docs-update` — Profession App Documentation

| Task | File | Detail |
|------|------|--------|
| Man pages for all 55 CLI tools | `docs/man/sigma-<app>.1` | One man page per profession app |
| India Stack API quick-reference | `wiki_repo/India-Stack-API-Reference.md` | ABDM, GSTN, UPI, DigiLocker endpoints |
| Profession onboarding guides | `wiki_repo/Profession-<App>-Guide.md` | Step-by-step for CA, doctor, farmer |
| Regulatory update policy | `wiki_repo/India-Regulatory-Updates.md` | How sigma-lex auto-updates profession apps |
| Video walkthroughs | `docs/demos/` | Screen-recorded demos for each app |

---

## `prepare-sigmaos-launch` — v15.1 Profession Launch Gates

Before v15.1 ships, minimum profession app criteria:

| Gate | Requirement |
|------|------------|
| sigma-agri | MSP lookup + PMFBY premium calc work offline |
| sigma-accounts | Post a sales voucher, generate GSTR-1 JSON |
| sigma-ca | Compute income tax for basic ITR-1 |
| sigma-pos | Generate UPI QR code for payment collection |
| sigma-health | Create ABHA health ID (ABDM sandbox) |
| sigma-gram | Record MGNREGS attendance |
| sigma-legal | Look up BNS 2023 section |
| sigma-edu | Mark attendance and generate report |
| DID credentials | CA + Doctor + Advocate DID credential QR |
| sigma-pkg | All 55 apps installable via `sigma-pkg install` |

---

## `gh-pages` — Profession Tools Website

```bash

# App gallery on sigmaos.dev:

sigma-site build --include profession-gallery
```

| Task | File | Detail |
|------|------|--------|
| Profession gallery page | `index.html` | Cards for all 55 apps with sector icons |
| Interactive MSP lookup | `sigma-web demo` | sigma-agri WASM in browser |
| GST calculator demo | `sigma-web demo` | sigma-ca WASM basic GST compute |
| India map: profession coverage | `index.html` | D3.js India map, hover shows apps by state |

---

## `master` — Stable Profession App Baseline

`master` mirrors `main`. Profession apps merge here after:

1. Core business logic `.cpp` body complete

2. India Stack API integrated (sandbox, then production)

3. Offline fallback for no-internet use

4. DID signature on all outputs

5. Man page written

6. sigma-pkg recipe signed + tested

---

## Master Profession App Status Table

| App | .cpp body | CLI | India API | Offline data | DID sig | .spkg |
|-----|-----------|-----|-----------|--------------|---------|-------|
| sigma-agri | ✅ partial | ✅ | ❌ | ✅ MSP table | ❌ | ❌ |
| sigma-edu | ⚠️ | ⚠️ | ❌ | ⚠️ | ❌ | ❌ |
| sigma-gov | ⚠️ | ⚠️ | ❌ | ❌ | ❌ | ❌ |
| sigma-labour | ⚠️ | ⚠️ | ❌ | ❌ | ❌ | ❌ |
| sigma-bank | ⚠️ | ⚠️ | ❌ | ❌ | ❌ | ❌ |
| sigma-realty | ⚠️ | ⚠️ | ❌ | ❌ | ❌ | ❌ |
| sigma-startup | ⚠️ | ⚠️ | ❌ | ❌ | ❌ | ❌ |
| sigma-ca | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| sigma-accounts | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| sigma-health | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| sigma-legal | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| sigma-pharma | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| sigma-pos | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| sigma-gram | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| sigma-hrms | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| sigma-police | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| All other 39 | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |

**Legend:** ✅ done · ⚠️ partial · ❌ not started

---

*See also: [India Profession Coverage](India-Profession-Coverage) · [India Business Strategy](India-Business-Strategy) · [CLI Commands Roadmap](CLI-Commands-Roadmap) · [Feature Branch Roadmap](Feature-Branch-Roadmap) · [Development Roadmap](Development-Roadmap)*
