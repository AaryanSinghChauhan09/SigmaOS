# ⚖️ Indian Legal Procedure Checklist (BNS, BNSS, BSA)

SigmaOS implements the absolute **Native Indian Judicial Shard** (`indian_law.c`). This checklist outlines the exact, updated procedural execution flow modeled after the new criminal laws: **Bharatiya Nyaya Sanhita (BNS)**, **Bharatiya Nagarik Suraksha Sanhita (BNSS)**, and **Bharatiya Sakshya Adhiniyam (BSA)**.

Using SigmaOS, forensic analysts, lawyers, and law enforcement can parse these procedures instantly via the `sigma_invoke indian_law` routine without querying a slow, external web database.

---

## 1. Information & FIR Registration (Zero-FI)

*Transitioning from CrPC Section 154 to **BNSS Section 173**.*

- [x] **Zero FIR Parsing:** Automatically validate if the complaint requires a Zero FIR regardless of police station jurisdiction constraint.
- [x] **E-FIR Logging Validation:** Cross-check the digital signature routing. Under BNSS, signatures must be verified within 3 days to establish the e-FIR.
- [x] **Preliminary Inquiry Flag:** SigmaOS ML algorithms parse the complaint text to determine if it falls under crimes punishable by 3-7 years (requiring a 14-day preliminary inquiry before formal registration).
- [x] **Information of Cognizable Offense:** Map the offense strictly against the updated **BNS Schedule**.

## 2. Investigation & Digital Forensics

*Transitioning from CrPC to **BNSS Data Search & Seizure**.*

- [x] **Forensic Videography Check:** If the crime carries a sentence of 7+ years, the SigmaOS shard mathematically flags that **mandatory videography** of the crime scene and evidence collection is required (BNSS Sec 176(3)).
- [x] **Amnesic Evidence Scrubbing:** Any local copy of seized digital evidence passed through SigmaOS is aggressively locked down utilizing the zero-dependency `sigma_sha256()` hashing system to prove chain-of-custody under **BSA Sec 61** (Electronic evidence).
- [x] **15-Day Custody Parsing:** Detect and validate police custody spanning up to 60/90 days in tranches, verifying timeline legality natively.

## 3. Arrest & Bail Heuristics

*BNS & BNSS Rights Validations.*

- [x] **Handcuffing Verification:** Cross-reference the arrestee’s status. BNSS Section 43 strictly defines when handcuffs are legal (e.g., repeating offenders, organized crime, terror).
- [x] **Information to Relative:** Verify the digital log proving the designated person was informed of the arrest.
- [x] **Bail Algorithm Processing:** For first-time offenders who have served 1/3rd of their maximum term (or 1/2 for others), natively prompt a statutory bail recommendation alert based on BNSS Sec 479 constraints.

## 4. Trial, Summons, & Prosecution

*Execution speeds drastically increased by BNSS timelines.*

- [x] **Digital Summons Generation:** The OS generates standard digital summons utilizing the native text buffer (BNSS Sec 70) without requiring a GUI.
- [x] **Ex-Parte / Absentee Trial:** Verify if the accused is intentionally evading trial for 90 days. If true, SigmaOS logs a procedural flag to commence trial in absentia (BNSS Sec 356).
- [x] **Judgment Timeline Watchdog:** The OS kernel establishes a daemon timer. Judgments must be pronounced within 30-45 days of concluding arguments.

## 5. Execution of Evidence (BSA)

*Replacing the Indian Evidence Act natively.*

- [x] **Digital Record Verification:** SigmaOS intrinsically signs digital evidence logs bypassing third-party certificate authorities using raw `SYS_WRITE` buffers.
- [x] **Primary Evidence Reclassification:** Log video recordings and CCTV feeds natively as **primary evidence** instead of secondary documentation under the new BSA rules.

---

### 💻 Invoking the Matrix via Omni Shell

To run a procedural check on an open case file in SigmaOS

```bash

# Example: Parse an FIR text file against BNS/BNSS requirements

sigma_invoke indian_law --audit_fir /vfs/case_files/FIR_102.txt

# Example: Run the hardware SHA-256 validation for Digital Evidence (BSA Sec 61)

sigma_invoke indian_law --validate_evidence /vfs/evidence/drive.img
```

The OS processes the complete legal flow offline, instantly, directly on silicon.
