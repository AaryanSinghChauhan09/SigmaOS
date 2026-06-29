# SigmaOS Extended Profession Tools — Round 35

14 additional India profession app headers covering Company Secretaries, SEBI brokers, aviation, food safety, mining, textiles, marine, forests, dental, veterinary, and trusts/NGOs. Every app targets a specific Indian regulatory body and replaces the current manual/Excel/external-software workflow with an OS-native solution.

---

## sigma-cs — Company Secretary Tools

3 lakh+ practicing CS professionals. The most under-digitised profession in India.

**Regulator:** ICSI / MCA / SEBI LODR / Companies Act 2013

| Feature | Regulation | Implementation |
|---|---|---|
| ROC filings (MGT-7, AOC-4, DIR-12, SH-7) | Companies Act 2013 | `sigma_cs_roc_filing_t` + MCA21 API |
| Secretarial Standards (SS-1, SS-2, SS-4) | ICSI | Board meeting notice + minutes |
| SEBI LODR compliance calendar | SEBI LODR 2015 | Quarterly deadline tracker |
| Board meeting management | SS-1 | 7-day notice, agenda, minutes, e-voting |
| FEMA reporting (FC-GPR, FC-TRS, APR) | FEMA 1999 | RBI/MCA eBiz portal integration |
| Secretarial audit (Form MR-3) | Companies Act §204 | Annual audit workflow |

```bash
sigma-cs roc file MGT-7 --cin U12345MH2020PTC123456
sigma-cs board-meeting agenda --company "XYZ Ltd" --date 2026-07-15
sigma-cs sebi lodr quarterly-compliance --quarter Q1-FY27
sigma-cs fema fc-gpr --cin <CIN> --investor-country US --amount 5000000
```

**Key differentiator:** Automatic compliance calendar — sigma-cs knows every ROC, SEBI, and FEMA deadline for a given company and alerts 30/15/7 days in advance.

---

## sigma-sebi — Capital Markets & Securities Professionals

SEBI regulates India's ₹400 lakh crore market. Every broker, RIA, and MFD currently uses fragmented Excel + web portals.

**Regulator:** SEBI / AMFI / NSDL / CDSL

| Feature | Regulation | Implementation |
|---|---|---|
| Peak margin compliance | SEBI Circular SEBI/HO/MRD2 | Daily peak margin tracking |
| Capital gains calculation (STCG/LTCG) | Finance Act 2024 | 20%/12.5% rates, ₹1.25L exemption |
| KYC / IPV tracking | SEBI KYC Reg | Annual IPV renewal alert |
| IA advice register | SEBI (IA) Regulations 2013 | Mandatory log of all advice |
| AMFI ARN + EUIN tracking | AMFI | Certificate expiry, CPD log |
| SCORES complaint filing | SEBI SCORES | Direct portal integration |
| Grandfathering (pre-2018) cost basis | Finance Act 2018 | Jan 31, 2018 fair market value |

```bash
sigma-sebi margin peak-report --client CLIENT001 --date today
sigma-sebi gains calculate --fy 2026-27 --demat-statement demat.pdf
sigma-sebi kyc verify --pan ABCDE1234F --dob 1990-01-15
sigma-sebi scores file --entity "XYZ Broker" --complaint complaint.txt
```

**Key differentiator:** Automatic LTCG grandfathering — buys before Jan 31, 2018 use the correct cost basis automatically. This is where every CA/investor makes mistakes.

---

## sigma-aviation — Aviation Professionals

DGCA regulates India's ₹1.5 lakh crore aviation sector with 700+ aircraft and 65,000+ licensed professionals.

**Regulator:** DGCA / AAI / BCAS / ICAO / STCW

| Feature | What it does |
|---|---|
| Pilot logbook | Digital log: PIC/SIC/IFR/night hours, currency check |
| STCW certificate tracker | Expiry alerts for all 10+ mandatory certs |
| FRMS (Fatigue Risk Management) | Duty hour limits per DGCA (FDTL rules) |
| Weather briefing | METAR + TAF + SIGMET + NOTAM in one view |
| Weight & Balance | Aircraft-type W&B calculation with CG envelope |
| AME maintenance records | Task card log, RTS certificate, CAME reference |
| SDR (Service Difficulty Report) | Direct DGCA filing |

```bash
sigma-aviation hours log --aircraft VT-ABC --hours 6.5 --date today
sigma-aviation weather briefing --from DEL --to BOM
sigma-aviation dgca license check --validity
sigma-aviation wb calculate --aircraft B737 --payload 15000
sigma-aviation frms check --planned-duty 14
```

**Key differentiator:** Single-view pre-flight briefing — weather + NOTAMs + fuel + W&B + crew FDTL status in one sigma-aviation preflight command.

---

## sigma-fssai — Food Safety & Restaurant Management

India has 7.5 million food businesses. FSSAI's 2024 mandatory allergen labeling caught most businesses off-guard.

**Regulator:** FSSAI / Food Safety and Standards Act 2006

| Feature | Regulation | Implementation |
|---|---|---|
| Licence type determination | FSSAI Licensing Regs 2011 | Auto-determine based on turnover/multi-state |
| Mandatory allergen labeling | FSSAI (Food Products Standards) 2024 | 10-allergen check on every menu item |
| HACCP CCP monitoring | Codex Alimentarius / FSSAI | IoT sensor integration for CCPs |
| Temperature log (cold chain) | FSSAI Food Safety | Hourly cold chain records |
| POS with GST | GST 5%/18% slabs | Restaurant billing with correct slab |
| Aggregator sync | ONDC/Swiggy/Zomato | Menu + order auto-sync |
| Recall management | FSSAI Recall Portal | Batch recall filing |
| Hygiene rating | Eat Right India | Self-assessment checklist |

```bash
sigma-fssai licence check --turnover 5000000
sigma-fssai haccp temperature log --zone Kitchen --temp 4
sigma-fssai allergen check --menu menu.json
sigma-fssai hygiene audit --rating-checklist
```

**Key differentiator:** The allergen declaration requirement (mandatory from 2024) is killing small restaurants who don't know about it. sigma-fssai makes it zero effort — just enter ingredients and it auto-generates the declaration.

---

## sigma-mining — Mining Professionals

India's ₹2.5 lakh crore mining sector with 1,500+ operational mines. Fatal accident reporting has a 2-hour window that most mines miss due to paperwork.

**Regulator:** DGMS / IBM / MMDR Act 2015 / PESO

| Feature | Regulation | What it does |
|---|---|---|
| Accident report (Form I) | Mines Act 1952 §23 | One-tap filing within 2-hour mandatory window |
| Blasting register | PESO Explosives Rules | Per-shot log with PPV measurement |
| HEMM maintenance log | DGMS | Mandatory equipment maintenance records |
| IBM monthly return | MMDR Act | Production + dispatch + royalty auto-calculation |
| DMF/NMET levy | MMDR §9B + 9C | Auto-calculates 10-30% of royalty |
| Environmental monitoring | EAC/CPCB | PM10, PM2.5, noise, vibration log |
| Mineral dispatch challan | MMDR | Electronic MDC generation |

```bash
sigma-mining accident report --type fatal --date today --location "Level 3"
sigma-mining blast log --explosive ANFO --qty 150kg --holes 12
sigma-mining ibm return --month 2026-06
sigma-mining royalty calculate --mineral iron-ore --state Odisha --qty 10000MT
```

**Key differentiator:** The 2-hour accident report requirement is a legal obligation that mine managers regularly miss because of remote locations and paper forms. sigma-mining creates a one-tap mobile filing with GPS coordinates that auto-generates Form I.

---

## sigma-textile — Textile & Fashion Professionals

India's ₹15 lakh crore textile sector — 2nd largest employer. Mandatory labeling under 2023 rules is widely non-compliant.

**Regulator:** Textile Commissioner / BIS / CITI / AEPC

| Feature | Regulation | What it does |
|---|---|---|
| Mandatory label compliance | Textile (Consumer Protection) Rules 2023 | 8-field mandatory label check |
| Handloom Mark application | Textile Commissioner | Online application + tracking |
| GI Tag management | GI Act 1999 | Application + renewal for Banarasi, Kanchipuram etc. |
| PM Vishwakarma scheme | MSME Ministry | Loan application (₹3L collateral-free) |
| Production order management | — | Style/size breakdown + fabric consumption |
| RoSCTL claim | DGFT | Export incentive calculation + filing |
| Marker efficiency calculation | — | Cutting room waste reduction |

```bash
sigma-textile label check --product shirt.json
sigma-textile handloom mark apply --weaver W001
sigma-textile production order --buyer "H&M" --style S001 --qty 5000
sigma-textile rosctl claim --shipping-bill SB123456 --fob-value 500000
```

---

## sigma-marine — Maritime Professionals

India has 7,500km of coastline and is the world's largest source of merchant marine officers. STCW revalidation lapses are career-ending.

**Regulator:** DG Shipping / MMD / STCW / IMO

| Feature | What it does |
|---|---|
| COC + STCW certificate tracker | Expiry alerts for all 10+ mandatory STCW certs |
| GMDSS log | Daily DSC watch record (mandatory) |
| Voyage planning | Distance, fuel, ETA, weather routing |
| Stability calculation | GM, trim, list — IMO minimum compliance |
| Bunker records | MARPOL Annex VI sulphur content compliance |
| Port dues calculator | All major Indian ports + foreign ports |
| INDSAR integration | Indian Seafarer Record portal |

```bash
sigma-marine stcw check --rank "Chief Officer" --expiry-check
sigma-marine stability --gm 1.5 --kG 7.2 --displacement 5000T
sigma-marine voyage plan --from INBOM --to SGSIN
sigma-marine bunker record --fuel VLSFO --qty 500MT
```

**Key differentiator:** sigma-marine is the only tool that tracks all 10+ STCW certificates simultaneously and alerts 6 months before each expiry. A lapsed STCW cert means the officer cannot sail — financially devastating.

---

## sigma-forest — Forest & Wildlife Officers

India's 700,000 sq km of forest is managed by 80,000 forest officers. M-STrIPES patrolling data, FRC claims, and fire reports are all done on paper today.

**Regulator:** MoEFCC / NTCA / FSI / WII / CAMPA

| Feature | Regulation | What it does |
|---|---|---|
| Forest Rights Claims (FRC) | FRA 2006 | Digital FRC filing + status tracking |
| M-STrIPES patrol log | NTCA | GPS-tracked patrol + wildlife observation |
| Forest fire report | Fire Manual | NASA FIRMS integration + incident report |
| FC Act diversion | FC Act 1980 | NPV + CAMPA levy calculation |
| WPA species database | WPA 1972 | Schedule I-VI species lookup |
| CITES permit check | CITES | Import/export permit requirement check |
| Carbon credit documentation | REDD+ / VCS | Carbon stock calculation + Verra prep |

```bash
sigma-forest frc claim --village "Rampur" --area 5.2-hectares
sigma-forest fire alert --district Bastar --severity high
sigma-forest patrol log --reserve "Corbett" --distance 12km
sigma-forest diversion npv --area 100ha --forest-type reserved --state MP
```

---

## sigma-trust — Religious Institutions & NGO Management

India has 3 million+ temples, mosques, churches, and gurudwaras. FCRA compliance has caused thousands of organisations to lose their registration.

**Regulator:** Charity Commissioner / FCRA / IT Dept (12A/80G) / Waqf Board

| Feature | Regulation | What it does |
|---|---|---|
| FCRA annual return (FC-4) | FCRA 2010 | Annual return due Sept 30 |
| 80G receipt generation | IT Act §80G | Form 10BE certificate to donor |
| Form 10BD aggregation | IT Act | Annual donation statement due May 31 |
| 12A/80G renewal tracking | IT Act §12AB | 5-year validity renewal alert |
| Hundi counting record | Charity Commissioner | Witness + count log |
| CSR fund receipt compliance | Companies Act §135 | CSR project tracking |
| Waqf Act compliance | Waqf Act 1995 | Waqf Board registration + audit |

```bash
sigma-trust fcra return FC4 --fy 2025-26
sigma-trust 80g receipt --donor "Ram Sharma" --amount 51000
sigma-trust donations hundi-count --date today
sigma-trust 12a renewal check --expiry
```

**Key differentiator:** The FCRA SBI NDLS bank account requirement (foreign contributions must only go through SBI New Delhi Main Branch) trips up every newly FCRA-registered NGO. sigma-trust validates this at registration time.

---

## Summary — New Profession App Headers (Round 35)

| App | Profession | Regulator | File |
|---|---|---|---|
| `sigma-cs` | Company Secretary | ICSI/MCA/SEBI | `userland/apps/sigma-cs/sigma_cs.h` |
| `sigma-sebi` | Stock broker/RIA/MFD | SEBI/AMFI | `userland/apps/sigma-sebi/sigma_sebi.h` |
| `sigma-aviation` | Pilot/AME/ATC | DGCA/AAI | `userland/apps/sigma-aviation/sigma_aviation.h` |
| `sigma-fssai` | Restaurant/food biz | FSSAI | `userland/apps/sigma-fssai/sigma_fssai.h` |
| `sigma-mining` | Mine manager/officer | DGMS/IBM/MMDR | `userland/apps/sigma-mining/sigma_mining.h` |
| `sigma-textile` | Weaver/garment mfr | Textile Commissioner | `userland/apps/sigma-textile/sigma_textile.h` |
| `sigma-marine` | Ship officer/AME | DG Shipping | `userland/apps/sigma-marine/sigma_marine.h` |
| `sigma-forest` | Forest/wildlife officer | MoEFCC/NTCA | `userland/apps/sigma-forest/sigma_forest.h` |
| `sigma-trust` | NGO/temple manager | FCRA/IT Dept | `userland/apps/sigma-trust/sigma_trust.h` |

Plus from previous round (already committed):
- `sigma-cs` Company Secretary header
- `sigma-sebi` SEBI professionals header
- `sigma-textile` Textile industry header
- `sigma-trust` Religious institutions header

---

*See also: [India Profession Coverage](India-Profession-Coverage) · [India Business Strategy](India-Business-Strategy) · [SigmaOS Vision for India](SigmaOS-Vision-India)*
