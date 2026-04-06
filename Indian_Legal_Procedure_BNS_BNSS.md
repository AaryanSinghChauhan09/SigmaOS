# SigmaOS Legal Portal: Indian Law Checklists (BNS, BNSS, BSA)

**Repository Synchronization Target**: `indian_law.c` Shard  
**Status**: ACTIVE as of Current Indian Court Standings (Post July 1, 2024 implementation of BNS, BNSS, and BSA)

This document serves as the master checklist to outline the exact execution steps, forms, and procedural necessities embedded within the `sigma law` execution module. This allows forensic professionals using SigmaOS to navigate the modern Indian Criminal Justice System locally without internet dependencies.

---

## 1. Information & FIR Registration (Sec 173 BNSS)

The Bharatiya Nagarik Suraksha Sanhita (BNSS) standardizes digital entry and preliminary inquiries.

### Procedures & Essentials

- **e-FIR (Electronic FIR)**: Information can be submitted electronically but MUST be followed by the informant's signature within 3 days to become an actionable FIR.
- **Zero FIR Protocol**: Can be registered irrespective of the area where the offense was committed. The SigmaOS shard automatically routes jurisdiction mapping to the correct station.
- **Preliminary Inquiry**: For offenses carrying 3 to 7 years imprisonment (e.g., cheating, certain frauds), the police now hold the statutory right to conduct a preliminary inquiry for up to **14 days** prior to formal FIR registration.
- **Form Requirement**: Entry must be made into the *State Crime Records Bureau (SCRB) General Diary (GD)* and the official FIR Book.

---

## 2. Arrest Protocols (Sec 35 - 43 BNSS)

Updates to arrest mechanics grant explicit permissions for complex crimes while mandating strict rights.

### Procedures & Essentials

- **Handcuffing Guidelines (Sec 43(3))**: Explicitly permitted when arresting habitual offenders, escaping convicts, or persons involved in organized crime, terrorism, or severe economic offenses.
- **Notice of Appearance (Sec 35)**: Replaces the old Sec 41A of CrPC. If arrest is not strictly required, a formal notice must be issued.
- **Information to Relatives (Sec 47)**: The arresting officer MUST inform the designated relative or friend immediately.
- **Form Requirement**:
  - **Arrest Memo**: Must be prepared at the time of arrest and attested by at least one witness (family member or locality respectable) and countersigned by the arrestee.
  - **Health Memo**: Mandatory medical examination by a registered medical practitioner within 24 hours (Sec 53 BNSS).

---

## 3. Search, Seizure & Digital Evidence (Sec 105 BNSS, Sec 61 BSA)

SigmaOS is particularly designed to interface with the stringent new digital evidence laws.

### Procedures & Essentials

- **Mandatory Videography**: Search and seizure operations MUST be recorded via audio-video electronic means. (The Sigma Omni-Media Engine supports native, offline cryptographic hashing of these videos).
- **Digital Evidence Admissibility (Sec 61 BSA)**: Electronic records are now primary evidence. However, they require a certificate detailing the device, exact timestamps, and cryptographic hash functions.
- **Form Requirement**:
  - **Seizure Memo/Panchnama**: Must detail all seized items.
  - **Sec 61/63 BSA Certificate**: For any digital device seized, the investigating forensic officer must attach a certificate validating the integrity (Hash Value: SHA-256) of the device state.

---

## 4. Remand and Police Custody (Sec 187 BNSS)

The structure of Police Custody (PC) has been heavily modified.

### Procedures & Essentials

- **Phased Custody**: Police Custody (maximum 15 days) can now be requested *in phases* during the initial 40 days (for offenses with up to 10 years punishment) or 60 days (for offenses with death/life imprisonment/10+ years), rather than being restricted to the first 15 days post-arrest.
- **Form Requirement**:
  - Remand Application detailing the necessity of phased custody and progress of the investigation.

---

## 5. Trial & Judgment Timelines

Strict timelines are enforced to crush pendency.

### Procedures & Essentials

- **Trial in Absentia (Sec 356 BNSS)**: If a proclaimed offender absconds to evade trial and there is no immediate prospect of arresting them, the trial can commence and conclude in their absence after 90 days from the framing of charges.
- **Judgement Timeline**: Judgments must be pronounced within **30 days** (extendable to 45 days) from the conclusion of arguments.
- **Victim Rights**: The victim has the right to be informed about the progress of the investigation within 90 days. Case withdrawal for offenses carrying 7+ years imprisonment requires giving the victim an opportunity to be heard.

---

## Integration into SigmaOS Omni-CLI

The `indian_law.c` Shard allows users to instantly generate the BSA Sec 61 certificates and pre-fill Arrest/Seizure Memos directly via the terminal:

```bash
# Generate a Sec 61 BSA digital evidence certificate for a seized hard drive
root@sigma:~# sigma law --generate-cert --device /dev/sdX1 --hash SHA256

# Pull exact arrest procedures for organized crime
root@sigma:~# sigma law --query "handcuffing procedure BNSS"
```
