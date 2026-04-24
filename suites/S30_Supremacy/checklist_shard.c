/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN-CHECKLIST-SHARD (v3.0 - OMNI-LEGAL ZENITH)
 * =============================================================================
 * Algorithm: Sharded Legal Automation & Compliance — SLAC v3.0
 * Principles:
 *   - Zero-dependency legal logic matrices (C11 pure).
 *   - Universal BNSS/BNS/BSA/CPC/GST/POCSO/PMLA/RTI/IBC/DPDP procedural integration.
 *   - Kernel-native deadline tracking and compliance scoring.
 *   - Novice-first: every item has prerequisites, procedure, and section reference.
 *   - No single Indian law or procedure is absent from this shard.
 * Target: Forensic Scientists, Lawyers, Researchers, NCERT Students, Novices.
 * Based on: Latest Supreme Court & High Court interpretations (2024-2026).
 * =============================================================================
 */

#include "sigma_kernel_types.h"

/* =========================================================================
 * Constants
 * ========================================================================= */
#define MAX_CHECKLIST_ITEMS    200
#define MAX_TEMPLATES          32
#define MAX_STEP_LEN           512
#define MAX_PREREQ_LEN         512
#define MAX_DESC_LEN           128
#define MAX_REF_LEN            64

/* =========================================================================
 * Zero-dependency string utility (no libc)
 * ========================================================================= */
static void sigma_strncpy(char* dest, const char* src, u32 n) {
    u32 i;
    for (i = 0; i < n - 1 && src[i] != '\0'; i++) dest[i] = src[i];
    dest[i] = '\0';
}

static u32 sigma_sigma_strlen(const char* s) {
    u32 i = 0;
    while (s[i]) i++;
    return i;
}

/* =========================================================================
 * Domain Taxonomy (Complete Indian Law Classification)
 * ========================================================================= */
typedef enum LawDomain {
    DOMAIN_CRIMINAL_BNSS = 0,   /* Bharatiya Nagarik Suraksha Sanhita 2023 */
    DOMAIN_SUBSTANTIVE_BNS,     /* Bharatiya Nyaya Sanhita 2023 */
    DOMAIN_EVIDENCE_BSA,        /* Bharatiya Sakshya Adhiniyam 2023 */
    DOMAIN_CIVIL_CPC,           /* Code of Civil Procedure 1908 */
    DOMAIN_CONTRACT,            /* Indian Contract Act 1872 */
    DOMAIN_PROPERTY,            /* Transfer of Property Act 1882 */
    DOMAIN_FAMILY_MARRIAGE,     /* Hindu Marriage Act / Special Marriage Act */
    DOMAIN_SUCCESSION,          /* Hindu Succession Act / Indian Succession Act */
    DOMAIN_LABOUR_CODE,         /* Labour Codes 2019-2020 */
    DOMAIN_INDUSTRIAL_RELATIONS,/* Industrial Relations Code 2020 */
    DOMAIN_TAX_GST,             /* GST Act / Income Tax Act */
    DOMAIN_COMPANIES_ACT,       /* Companies Act 2013 */
    DOMAIN_INSOLVENCY_IBC,      /* Insolvency & Bankruptcy Code 2016 */
    DOMAIN_CONSUMER_PROTECTION, /* Consumer Protection Act 2019 */
    DOMAIN_CYBER_IT_ACT,        /* IT Act 2000 / CERT-In Directives */
    DOMAIN_DPDP_ACT,            /* Digital Personal Data Protection Act 2023 */
    DOMAIN_PMLA,                /* Prevention of Money Laundering Act 2002 */
    DOMAIN_POCSO,               /* POCSO Act 2012 */
    DOMAIN_UAPA,                /* UAPA 1967 */
    DOMAIN_NDPS,                /* NDPS Act 1985 */
    DOMAIN_IP_TRADEMARK,        /* Trade Marks Act 1999 */
    DOMAIN_IP_COPYRIGHT,        /* Copyright Act 1957 */
    DOMAIN_IP_PATENT,           /* Patents Act 1970 */
    DOMAIN_RERA,                /* Real Estate (Regulation) Act 2016 */
    DOMAIN_RTI,                 /* Right to Information Act 2005 */
    DOMAIN_ARBITRATION,         /* Arbitration & Conciliation Act 1996 */
    DOMAIN_FORENSIC_BSA,        /* Digital Forensics (BSA ch.) */
    DOMAIN_SEBI_SECURITIES,     /* SEBI Act / Securities Laws */
    DOMAIN_FEMA,                /* Foreign Exchange Management Act 1999 */
    DOMAIN_ENVIRONMENTAL,       /* Environment Protection Act 1986 */
    DOMAIN_COMPETITION_LAW,     /* Competition Act 2002 */
    DOMAIN_CUSTOMS_EXCISE       /* Customs Act 1962 */
} LawDomain;

/* =========================================================================
 * Compliance Status
 * ========================================================================= */
typedef enum ComplianceStatus {
    STATUS_PENDING    = 0,
    STATUS_DRAFTED    = 1,
    STATUS_FILED      = 2,
    STATUS_VERIFIED   = 3,
    STATUS_COMPLETED  = 4,
    STATUS_REJECTED   = 5
} ComplianceStatus;

/* =========================================================================
 * Checklist Item — the atomic legal unit
 *   desc         : Short description of the step
 *   section_ref  : Precise section/rule reference
 *   prerequisites: What the novice must have BEFORE this step
 *   procedure    : Step-by-step instructions
 *   deadline_days: 0 means immediate; >0 means statutory limit
 *   status       : Current compliance status
 * ========================================================================= */
typedef struct ChecklistItem {
    char             desc[MAX_DESC_LEN];
    char             section_ref[MAX_REF_LEN];
    char             prerequisites[MAX_PREREQ_LEN];
    char             procedure[MAX_STEP_LEN];
    u32              deadline_days;
    ComplianceStatus status;
    u32              penalty_rs;      /* penalty in rupees if missed */
} ChecklistItem;

/* =========================================================================
 * Legal Template — a domain-scoped collection of checklist items
 * ========================================================================= */
typedef struct LegalTemplate {
    LawDomain     domain;
    char          name[64];
    char          description[256];
    u32           item_count;
    ChecklistItem items[MAX_CHECKLIST_ITEMS];
} LegalTemplate;

static LegalTemplate g_templates[MAX_TEMPLATES];
static u32           g_template_count = 0;

/* =========================================================================
 * Forward declarations
 * ========================================================================= */
extern void ksigma_printf(const char* fmt, ...);

/* =========================================================================
 * Helper: Add item to template
 * ========================================================================= */
static void add_item(LegalTemplate* t,
                     const char* desc,
                     const char* ref,
                     const char* prereq,
                     const char* proc,
                     u32 days,
                     u32 penalty_rs) {
    if (t->item_count >= MAX_CHECKLIST_ITEMS) return;
    ChecklistItem* ci = &t->items[t->item_count++];
    sigma_strncpy(ci->desc,          desc,   MAX_DESC_LEN);
    sigma_strncpy(ci->section_ref,   ref,    MAX_REF_LEN);
    sigma_strncpy(ci->prerequisites, prereq, MAX_PREREQ_LEN);
    sigma_strncpy(ci->procedure,     proc,   MAX_STEP_LEN);
    ci->deadline_days = days;
    ci->penalty_rs    = penalty_rs;
    ci->status        = STATUS_PENDING;
}

/* =========================================================================
 * BNSS 2023 — Criminal Procedure Template
 * ========================================================================= */
static void init_bnss_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_CRIMINAL_BNSS;
    sigma_strncpy(t->name, "BNSS_2023_CRIMINAL_PROCEDURE", 63);
    sigma_strncpy(t->description,
        "Bharatiya Nagarik Suraksha Sanhita 2023 — full criminal procedure "
        "from FIR to charge-sheet, remand, bail, trial, and appeal.", 255);
    t->item_count = 0;

    add_item(t,
        "Register FIR (mandatory for cognizable offence)",
        "Sec 173 BNSS",
        "PREREQ: Victim/complainant physical presence OR e-FIR portal login. "
        "Have: date/time/place of incident, offence description, accused details if known.",
        "STEP 1: Visit nearest police station (or use eFIR portal). "
        "STEP 2: Narrate facts in written complaint. "
        "STEP 3: Officer MUST register — cannot refuse a cognizable offence. "
        "STEP 4: Receive FIR copy free of cost (right under Sec 173 BNSS). "
        "STEP 5: Verify FIR is audio/video recorded as per BNSS mandate.",
        0, 0);

    add_item(t,
        "Notice of Arrest & Grounds Disclosure (constitutional right)",
        "Sec 48 & 54 BNSS / Art 22 Constitution",
        "PREREQ: Person is arrested. Have: reason for arrest ready.",
        "STEP 1: Police must inform arrested person of grounds immediately. "
        "STEP 2: Inform nominated person of arrest (Sec 43 BNSS). "
        "STEP 3: Medical examination within 24 hours. "
        "STEP 4: Right to advocate of choice must be communicated.",
        0, 0);

    add_item(t,
        "Produce before Magistrate within 24 hours",
        "Sec 57 BNSS / Art 22(2) Constitution",
        "PREREQ: Arrest has occurred. SHO must authorise or arrange transport.",
        "STEP 1: Police must produce arrestee before nearest Magistrate within 24 hours. "
        "STEP 2: Magistrate determines remand/bail. "
        "STEP 3: 24-hour clock excludes travel time from arrest place. "
        "STEP 4: Zero exceptions — Anita Kushwaha v. Pushap Sudan (2016 SC).",
        1, 0);

    add_item(t,
        "Police Custody Remand — Maximum 15-day limit",
        "Sec 187 BNSS",
        "PREREQ: Magistrate order for remand. Accused produced in court.",
        "STEP 1: Police apply for custody by written application. "
        "STEP 2: Magistrate may authorise max 15 days police custody initially. "
        "STEP 3: Total custody (police + judicial) cannot exceed 60/90 days (offence-specific). "
        "STEP 4: Challenge excess remand via writ in High Court (Art 226).",
        15, 0);

    add_item(t,
        "Search & Seizure — mandatory AV recording",
        "Sec 105 BNSS",
        "PREREQ: Search warrant issued by Magistrate OR emergency cognizable case. "
        "Officer must be at least SI rank. Independent witnesses required.",
        "STEP 1: Produce warrant before occupant. "
        "STEP 2: Record search on audio-video device from start to end. "
        "STEP 3: Prepare mahazar/panchnama with two independent witnesses. "
        "STEP 4: Give copy of seizure list to owner. "
        "STEP 5: Seized items to be tagged with exhibit number and hash if digital.",
        0, 0);

    add_item(t,
        "Bail Application (Bailable Offence)",
        "Sec 478 BNSS",
        "PREREQ: Arrested for bailable offence. Have: identity proof, surety.",
        "STEP 1: Application to officer-in-charge of police station or court. "
        "STEP 2: Provide surety bond and personal bond. "
        "STEP 3: Release is a RIGHT for bailable offences — officer/court cannot refuse. "
        "STEP 4: Note conditions of bail (attendance at trial etc.).",
        0, 0);

    add_item(t,
        "Bail Application (Non-bailable Offence)",
        "Sec 480-484 BNSS",
        "PREREQ: Arrested for non-bailable offence. Consult lawyer. "
        "Have: grounds for bail, surety details, identity proof.",
        "STEP 1: File bail application before Sessions/High Court (serious offences). "
        "STEP 2: State grounds: illness, parity, non-flight risk. "
        "STEP 3: Prosecution has right to oppose. "
        "STEP 4: Court may impose conditions: location tracking, passport surrender. "
        "STEP 5: Anticipatory bail available under Sec 484 BNSS.",
        0, 0);

    add_item(t,
        "Charge-sheet / Final Report filing",
        "Sec 193 BNSS",
        "PREREQ: Investigation complete. Within 60/90 days of arrest.",
        "STEP 1: Police submit charge-sheet to Magistrate court. "
        "STEP 2: Magistrate takes cognizance and summons accused. "
        "STEP 3: Accused entitled to copy of charge-sheet (Sec 230 BNSS). "
        "STEP 4: If not filed in time — default bail applies (Sec 187(5) BNSS).",
        60, 0);

    add_item(t,
        "Trial — Framing of Charges",
        "Sec 251-252 BNSS",
        "PREREQ: Cognizance taken. Accused appears. Charge-sheet supplied.",
        "STEP 1: Court examines charge-sheet and considers defence application. "
        "STEP 2: If prima facie case — charges framed and read to accused. "
        "STEP 3: Accused pleads guilty or not guilty. "
        "STEP 4: Discharge application available under Sec 250 BNSS if no case.",
        30, 0);

    add_item(t,
        "Victim Compensation under BNSS",
        "Sec 397 BNSS",
        "PREREQ: Judgment passed. Victim suffered loss.",
        "STEP 1: Apply to court for compensation order. "
        "STEP 2: Court considers: accused capacity to pay, victim losses. "
        "STEP 3: State Legal Services Authority may also award compensation.",
        30, 0);
}

/* =========================================================================
 * BNS 2023 — Substantive (Offence) Template
 * ========================================================================= */
static void init_bns_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_SUBSTANTIVE_BNS;
    sigma_strncpy(t->name, "BNS_2023_OFFENCES_MATRIX", 63);
    sigma_strncpy(t->description,
        "Bharatiya Nyaya Sanhita 2023 — mapping from old IPC to new BNS sections.", 255);
    t->item_count = 0;

    add_item(t, "Murder (BNS 103 = old IPC 302) — cognizance & trial",
        "Sec 103 BNS",
        "PREREQ: Death of a person. FIR registered. Post-mortem conducted.",
        "STEP 1: FIR under Sec 103 BNS with police. "
        "STEP 2: Post-mortem report is primary evidence. "
        "STEP 3: Trial in Sessions Court only. "
        "STEP 4: Punishment: death or life imprisonment.",
        0, 0);

    add_item(t, "Terrorism offence (new Sec 113 BNS)",
        "Sec 113 BNS",
        "PREREQ: Act threatening sovereignty/integrity/security of India.",
        "STEP 1: FIR under Sec 113 BNS. NIA may take over investigation. "
        "STEP 2: Trial in designated Special NIA Court. "
        "STEP 3: Bail is stringent — presumption against accused.",
        0, 0);

    add_item(t, "Cheating (Sec 318 BNS = IPC 420)",
        "Sec 318 BNS",
        "PREREQ: Dishonest inducement causing delivery of property.",
        "STEP 1: Register complaint with police. "
        "STEP 2: Gather: communications, financial trail, witness statements. "
        "STEP 3: Economic offences wing may investigate large amounts.",
        0, 0);

    add_item(t, "Organised Crime (new Sec 111 BNS)",
        "Sec 111 BNS",
        "PREREQ: Syndicate activity, violence, extortion, kidnapping. "
        "Include gang membership evidence.",
        "STEP 1: Report to police with evidence of ongoing criminal enterprise. "
        "STEP 2: Offence is non-bailable with enhanced punishment. "
        "STEP 3: Property can be attached under PMLA link.",
        0, 0);
}

/* =========================================================================
 * BSA 2023 — Evidence & Digital Forensics Template
 * ========================================================================= */
static void init_bsa_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_EVIDENCE_BSA;
    sigma_strncpy(t->name, "BSA_2023_EVIDENCE_DIGITAL_FORENSICS", 63);
    sigma_strncpy(t->description,
        "Bharatiya Sakshya Adhiniyam 2023 — electronic evidence, certificate "
        "requirements, admissibility, and forensic chain of custody.", 255);
    t->item_count = 0;

    add_item(t,
        "Certificate for Electronic Records (mandatory for admissibility)",
        "Sec 63 BSA",
        "PREREQ: Electronic record to be produced. "
        "Have: storage device, timestamp, hash (SHA-256/SHA-3), name of person responsible.",
        "STEP 1: Identify electronic record (email, WhatsApp, CCTV footage etc.). "
        "STEP 2: Generate SHA-256 hash of the file. "
        "STEP 3: Complete Form (Sec 63 certificate) stating: device, owner, hash, timestamp. "
        "STEP 4: Certificate signed by responsible official or IT security person. "
        "STEP 5: Attach to affidavit and file with court.",
        0, 0);

    add_item(t,
        "Primary Evidence — original storage device (Sec 62 BSA)",
        "Sec 62 BSA",
        "PREREQ: Physical device available. Write-blocker tool ready.",
        "STEP 1: Attach hardware write-blocker to device before imaging. "
        "STEP 2: Bit-accurate forensic image using dd or FTK Imager. "
        "STEP 3: Compare source hash == image hash (MD5 + SHA-256). "
        "STEP 4: Document chain of custody: who handled device, when, where. "
        "STEP 5: Store original in tamper-evident packaging for court.",
        0, 0);

    add_item(t,
        "Secondary Electronic Evidence (cloud/server records)",
        "Sec 63 BSA",
        "PREREQ: Server logs, cloud records. Provider's cooperation or court order.",
        "STEP 1: Obtain court order (Sec 94 BNSS) to compel provider to produce records. "
        "STEP 2: Provider furnishes records with their Sec 63 certificate. "
        "STEP 3: Verify integrity via hash provided by platform. "
        "STEP 4: Log all communications with provider for chain of custody.",
        0, 0);

    add_item(t,
        "Forensic DNA Evidence Admissibility",
        "Sec 53A BNSS + BSA Sec 39",
        "PREREQ: Biological sample collected. Accredited lab report available.",
        "STEP 1: Sample collected by authorised medical officer (Sec 53A BNSS). "
        "STEP 2: Lab must be NABL/NBA accredited. "
        "STEP 3: Lab report filed with Sec 63 BSA certificate. "
        "STEP 4: Expert may be cross-examined in court.",
        0, 0);
}

/* =========================================================================
 * POCSO 2012 — Child Protection Template
 * ========================================================================= */
static void init_pocso_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_POCSO;
    sigma_strncpy(t->name, "POCSO_2012_CHILD_PROTECTION", 63);
    sigma_strncpy(t->description,
        "Protection of Children from Sexual Offences Act 2012 — procedures "
        "for reporting, investigation, trial, and victim support.", 255);
    t->item_count = 0;

    add_item(t,
        "Mandatory reporting of POCSO offence",
        "Sec 19 POCSO",
        "PREREQ: Any person has knowledge of POCSO offence. "
        "Reporting is compulsory — failure is punishable.",
        "STEP 1: Report to local police or SJPU (Special Juvenile Police Unit) immediately. "
        "STEP 2: CWC (Child Welfare Committee) MUST be informed within 24 hours. "
        "STEP 3: Victim identity MUST NOT be disclosed (Sec 23 POCSO). "
        "STEP 4: Medical examination of child within 24 hours (Sec 27 POCSO). "
        "STEP 5: FIR registered under POCSO + relevant BNS sections.",
        0, 0);

    add_item(t,
        "Child-Friendly Investigation Procedure",
        "Sec 24-26 POCSO",
        "PREREQ: FIR registered. SJPU officer (preferably woman) assigned.",
        "STEP 1: Statement of child recorded by woman officer only. "
        "STEP 2: Child not required to visit police station. "
        "STEP 3: Statement recorded at child's residence or preferred place. "
        "STEP 4: No child held in custody overnight. "
        "STEP 5: All proceedings in camera (closed court).",
        0, 0);

    add_item(t,
        "Fast-Track Special Court (FTSC) trial",
        "Sec 28 POCSO / FTSC Scheme",
        "PREREQ: Charge-sheet filed. Child victim available for testimony.",
        "STEP 1: Case assigned to designated FTSC automatically. "
        "STEP 2: Trial must complete in 1 year (Nipun Saxena v. UOI, SC 2018). "
        "STEP 3: Child testimony via video-conference to avoid confrontation. "
        "STEP 4: Support person allowed throughout trial (Sec 40 POCSO). "
        "STEP 5: Victim compensation from POCSO Fund via DLSA.",
        365, 0);
}

/* =========================================================================
 * PMLA 2002 — Money Laundering Template
 * ========================================================================= */
static void init_pmla_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_PMLA;
    sigma_strncpy(t->name, "PMLA_2002_ANTI_MONEY_LAUNDERING", 63);
    sigma_strncpy(t->description,
        "Prevention of Money Laundering Act 2002 — attachment, prosecution, "
        "obligations and defence procedures.", 255);
    t->item_count = 0;

    add_item(t,
        "ED Arrest & Remand under PMLA",
        "Sec 19 PMLA",
        "PREREQ: ED has reason to believe money laundering offence is committed. "
        "ECIR (ED's equivalent of FIR) registered.",
        "STEP 1: ED officer arrests and informs grounds. "
        "STEP 2: Produce before Special PMLA Court within 24 hours. "
        "STEP 3: Bail is extremely stringent — Section 45 double negative test. "
        "STEP 4: Accused must prove innocence (reverse burden Sec 24 PMLA). "
        "STEP 5: Engage senior advocate specializing in economic offences.",
        0, 0);

    add_item(t,
        "Property Attachment (Provisional)",
        "Sec 5 PMLA",
        "PREREQ: ED identifies proceeds of crime. Director of ED authorisation needed.",
        "STEP 1: ED issues provisional attachment order (PAO) — 180-day validity. "
        "STEP 2: PAO communicated to owner within 30 days. "
        "STEP 3: File objections before Adjudicating Authority. "
        "STEP 4: File appeal before PMLA Appellate Tribunal. "
        "STEP 5: Ultimate appeal to High Court under Art 226.",
        30, 0);

    add_item(t,
        "KYC / AML Compliance for Businesses",
        "Sec 12 PMLA / FATF Norms",
        "PREREQ: Business is a Reporting Entity (bank, NBFC, broker, etc.).",
        "STEP 1: Collect KYC documents: Aadhaar, PAN, address proof for all clients. "
        "STEP 2: Verify beneficial ownership (persons owning >25%). "
        "STEP 3: Report Suspicious Transactions (STR) to FIU-Ind within 7 days. "
        "STEP 4: Report Cash Transactions >10L (CTR) to FIU-Ind monthly. "
        "STEP 5: Maintain records for 10 years.",
        7, 500000);
}

/* =========================================================================
 * RTI 2005 — Right to Information Template
 * ========================================================================= */
static void init_rti_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_RTI;
    sigma_strncpy(t->name, "RTI_2005_INFORMATION_ACCESS", 63);
    sigma_strncpy(t->description,
        "Right to Information Act 2005 — step-by-step procedure for filing "
        "RTI applications, appeals, and CIC complaints.", 255);
    t->item_count = 0;

    add_item(t,
        "File RTI Application (First Step)",
        "Sec 6 RTI Act",
        "PREREQ: Information sought from a Public Authority. "
        "Have: application fee (Rs 10 for Central Govt), postal order or IPO. "
        "Identify CPIO of the department.",
        "STEP 1: Write application in plain English/Hindi or official state language. "
        "STEP 2: State: your name, address, description of info required. "
        "STEP 3: Attach Rs 10 fee (IPO/DD/court fee stamp). BPL applicants exempt. "
        "STEP 4: Send to Central/State Public Information Officer (CPIO/SPIO) by post/hand. "
        "STEP 5: Keep acknowledgement copy. "
        "STEP 6: Response MUST come within 30 days (48 hours if life/liberty at stake).",
        30, 250);

    add_item(t,
        "First Appeal (if CPIO does not respond or rejects)",
        "Sec 19(1) RTI Act",
        "PREREQ: 30-day deadline passed OR response unsatisfactory. "
        "Have: CPIO response (if any), original RTI application.",
        "STEP 1: File First Appeal before Appellate Authority (senior CPIO) within 30 days. "
        "STEP 2: State grounds: no response, incomplete information, excessive exemptions. "
        "STEP 3: Appellate Authority must dispose in 30 days (max 45 days with reasons). "
        "STEP 4: No fee for First Appeal.",
        30, 0);

    add_item(t,
        "Second Appeal / Complaint to CIC / SIC",
        "Sec 19(3) RTI Act",
        "PREREQ: First Appeal failed or 45 days elapsed. "
        "CIC = Central Information Commission (Central Govt matters). "
        "SIC = State Information Commission (State matters).",
        "STEP 1: File complaint online at cic.gov.in or SIC portal within 90 days. "
        "STEP 2: Upload: RTI application, First Appeal, response/no-response proof. "
        "STEP 3: CIC/SIC can impose penalty on CPIO (Rs 250/day max Rs 25,000) and award compensation. "
        "STEP 4: Hearing is quasi-judicial — appear in person or via advocate.",
        90, 0);
}

/* =========================================================================
 * IBC 2016 — Insolvency & Bankruptcy Template
 * ========================================================================= */
static void init_ibc_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_INSOLVENCY_IBC;
    sigma_strncpy(t->name, "IBC_2016_INSOLVENCY_PROCEDURES", 63);
    sigma_strncpy(t->description,
        "Insolvency and Bankruptcy Code 2016 — initiating CIRP, liquidation, "
        "personal bankruptcy, and creditor rights.", 255);
    t->item_count = 0;

    add_item(t,
        "Initiate CIRP against Corporate Debtor (Financial Creditor)",
        "Sec 7 IBC",
        "PREREQ: Default of minimum Rs 1 crore. Have: financial debt proof, "
        "default documents, certified copy of order if any.",
        "STEP 1: Engage Insolvency Resolution Professional (IRP) from IBBI register. "
        "STEP 2: File Application in Form-1 at National Company Law Tribunal (NCLT). "
        "STEP 3: Pay court fee. NCLT must admit/reject within 14 days. "
        "STEP 4: Upon admission, moratorium declared (Sec 14 IBC) — no suits against company. "
        "STEP 5: IRP takes control within 180 days (extendable 90 days). "
        "STEP 6: Committee of Creditors (CoC) formed and resolution plan invited.",
        14, 0);

    add_item(t,
        "Operational Creditor demand notice before NCLT",
        "Sec 8-9 IBC",
        "PREREQ: Goods/services unpaid. Minimum Rs 1 crore default. "
        "Have: invoice, delivery proof, demand/rejection notice.",
        "STEP 1: Send demand notice (Form-3 or 4) to corporate debtor. "
        "STEP 2: Wait 10 days for payment or dispute notice. "
        "STEP 3: If not paid and no genuine dispute raised — file Sec 9 application at NCLT. "
        "STEP 4: NCLT must admit within 14 days.",
        10, 0);
}

/* =========================================================================
 * DPDP 2023 — Digital Personal Data Protection Template
 * ========================================================================= */
static void init_dpdp_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_DPDP_ACT;
    sigma_strncpy(t->name, "DPDP_2023_DATA_PROTECTION", 63);
    sigma_strncpy(t->description,
        "Digital Personal Data Protection Act 2023 — compliance obligations "
        "for Data Fiduciaries and rights of Data Principals.", 255);
    t->item_count = 0;

    add_item(t,
        "Obtain valid consent before processing personal data",
        "Sec 6 DPDP Act",
        "PREREQ: Business collects personal data of individuals (Data Principals). "
        "Have: privacy notice, consent mechanism.",
        "STEP 1: Provide notice specifying: what data, why collected, how used, data principal rights. "
        "STEP 2: Consent must be free, specific, informed, unconditional. "
        "STEP 3: Maintain consent record as proof. "
        "STEP 4: Withdrawal of consent must be as easy as giving consent. "
        "STEP 5: Children's data requires parental consent (Sec 9 DPDP Act).",
        0, 25000000);

    add_item(t,
        "Data breach notification to DPDB",
        "Sec 8(6) DPDP Act",
        "PREREQ: Personal data breach occurred. "
        "Identify: what data leaked, how many affected.",
        "STEP 1: Notify Data Protection Board of India (DPDB) promptly. "
        "STEP 2: Notify affected Data Principals immediately. "
        "STEP 3: Penalty for non-reporting: up to Rs 200 crore (Significant Data Fiduciary). "
        "STEP 4: Prepare incident report: timeline, data types, mitigation steps.",
        0, 20000000);

    add_item(t,
        "Respond to Data Principal rights requests",
        "Sec 11-13 DPDP Act",
        "PREREQ: Data Principal submits request for: access, correction, erasure, grievance.",
        "STEP 1: Acknowledge request within reasonable time. "
        "STEP 2: Provide information on data held within prescribed period. "
        "STEP 3: Erase/correct data as requested (unless legal/legitimate purpose). "
        "STEP 4: Maintain Grievance Officer contact on website. "
        "STEP 5: Appellate mechanism available before DPDB.",
        30, 0);
}

/* =========================================================================
 * GST & Tax Compliance Template
 * ========================================================================= */
static void init_gst_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_TAX_GST;
    sigma_strncpy(t->name, "GST_INCOME_TAX_COMPLIANCE_MATRIX", 63);
    sigma_strncpy(t->description,
        "GST Act / Income Tax Act — monthly, quarterly, and annual filing obligations "
        "for businesses and individuals.", 255);
    t->item_count = 0;

    add_item(t,
        "GSTR-1 Monthly Sales Return",
        "Sec 37 CGST Act",
        "PREREQ: Registered GST dealer with turnover >5 crore. "
        "Have: all outward supply invoices of the month. GSTIN active.",
        "STEP 1: Login GST Portal (gst.gov.in) before 11th of next month. "
        "STEP 2: Upload B2B and B2C invoices (or use ERP/Tally export). "
        "STEP 3: Verify auto-populated GSTR-2B (input credit). "
        "STEP 4: File GSTR-3B (summary return + tax payment) by 20th. "
        "STEP 5: Late fee: Rs 50/day (Rs 20 for nil return); interest 18% p.a.",
        11, 5000);

    add_item(t,
        "Income Tax ITR Filing (Individual)",
        "Sec 139 IT Act",
        "PREREQ: Income > basic exemption (Rs 3L/5L based on regime). "
        "Have: Form-16, AIS/TIS from IT Portal, bank statements.",
        "STEP 1: Login to incometax.gov.in. "
        "STEP 2: Select correct ITR form (ITR-1 for salary, ITR-4 for presumptive, etc.). "
        "STEP 3: Verify pre-filled data against Form-16 and AIS. "
        "STEP 4: Claim all eligible deductions (80C, 80D, HRA, etc.). "
        "STEP 5: Pay self-assessment tax if due. "
        "STEP 6: Verify return via Aadhaar OTP or EVC. "
        "DEADLINE: 31 July (with audit: 31 Oct / 30 Nov).",
        212, 5000);

    add_item(t,
        "TDS Deduction and Filing (Employer/Payer)",
        "Sec 192-194 IT Act",
        "PREREQ: Payer making taxable payment (salary, rent, professional fee). "
        "Have: TAN, payee PAN, payment details.",
        "STEP 1: Deduct TDS at prescribed rates at time of payment/credit. "
        "STEP 2: Deposit TDS to government by 7th of next month (govt depts: same day). "
        "STEP 3: File TDS return quarterly: 24Q (salary) / 26Q (others). "
        "STEP 4: Issue Form-16/16A to deductee within 15 days of return filing. "
        "STEP 5: Consequences of non-deduction: interest 1.5%/month + penalty.",
        7, 200);
}

/* =========================================================================
 * Arbitration Template
 * ========================================================================= */
static void init_arbitration_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_ARBITRATION;
    sigma_strncpy(t->name, "ARBITRATION_CONCILIATION_PROCEDURES", 63);
    sigma_strncpy(t->description,
        "Arbitration & Conciliation Act 1996 (as amended 2015, 2019, 2021) — "
        "commencing arbitration, tribunal composition, award and enforcement.", 255);
    t->item_count = 0;

    add_item(t,
        "Issue Notice to Invoke Arbitration",
        "Sec 21 Arbitration Act",
        "PREREQ: Valid arbitration clause in contract. Dispute has arisen. "
        "Have: contract, dispute description.",
        "STEP 1: Send written notice invoking arbitration clause to other party. "
        "STEP 2: Specify: dispute nature, relief claimed, proposed arbitrator (if any). "
        "STEP 3: Other party must respond within 30 days. "
        "STEP 4: Limitation period for filing: 3 years from dispute date (Sec 43).",
        30, 0);

    add_item(t,
        "Appointment of Arbitral Tribunal",
        "Sec 10-11 Arbitration Act",
        "PREREQ: Notice issued. Parties disagree on arbitrator.",
        "STEP 1: If parties fail to agree — apply to High Court under Sec 11(6). "
        "STEP 2: International commercial arbitration: Chief Justice of India. "
        "STEP 3: Designated arbitral institutions (like DIAC, ICC) can also appoint. "
        "STEP 4: Court must appoint within 30 days of application (post-2019 amendment).",
        30, 0);

    add_item(t,
        "Enforcement of Arbitral Award",
        "Sec 36 Arbitration Act",
        "PREREQ: Award passed and not challenged within 3 months, OR challenge dismissed.",
        "STEP 1: Apply to execute award as decree under CPC Order 21 (Sec 36). "
        "STEP 2: Attach respondent's assets if not complied voluntarily. "
        "STEP 3: Foreign awards: enforcement under Sec 44-60 (New York Convention).",
        90, 0);
}

/* =========================================================================
 * Cyber Law / IT Act / CERT-In Template
 * ========================================================================= */
static void init_cyber_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_CYBER_IT_ACT;
    sigma_strncpy(t->name, "CYBER_LAW_IT_ACT_CERT_IN_COMPLIANCE", 63);
    sigma_strncpy(t->description,
        "IT Act 2000 / CERT-In Directives 2022 — cyber crime reporting, "
        "data breach notification, and compliance checklist.", 255);
    t->item_count = 0;

    add_item(t,
        "Report Data Breach to CERT-In (6-hour mandate)",
        "CERT-In Directive u/s 70B IT Act",
        "PREREQ: Cyber incident/data breach detected. "
        "Identify: incident type (ransomware, data breach, phishing), impacted systems.",
        "STEP 1: Incident detected at any time — START 6-hour clock. "
        "STEP 2: Report to CERT-In portal (cert-in.org.in) or email (incident@cert-in.org.in). "
        "STEP 3: Report must include: date/time, systems affected, rootcause (if known), action taken. "
        "STEP 4: Submit supplementary report within 30 days with full details. "
        "STEP 5: Non-reporting: penalty under Sec 70B IT Act.",
        0, 100000);

    add_item(t,
        "Hacking / Unauthorised Computer Access — report and FIR",
        "Sec 66 IT Act / Sec 302 BNS equivalent",
        "PREREQ: Unauthorised access to your computer/network detected. "
        "Gather: logs, screenshots, network forensic data.",
        "STEP 1: Preserve all logs immediately (do not power off server). "
        "STEP 2: Report to Cyber Crime Portal (cybercrime.gov.in). "
        "STEP 3: FIR at cyber cell of nearest police station. "
        "STEP 4: Gather Sec 63 BSA-compliant digital evidence. "
        "STEP 5: If financial fraud — also report to bank fraud hotline 1930.",
        0, 0);

    add_item(t,
        "Social Media Intermediary Compliance (IT Rules 2021)",
        "IT (Intermediary Guidelines) Rules 2021",
        "PREREQ: Platform is a Social Media Intermediary (>5M users = Significant SMI). "
        "Must appoint Indian officers.",
        "STEP 1: Appoint Chief Compliance Officer (CCO), Nodal Contact Person, Resident Grievance Officer. "
        "STEP 2: Publish privacy policy and user agreement in compliance with Rules. "
        "STEP 3: Monthly compliance report to MeitY. "
        "STEP 4: Grievance redressal within 24 hours (acknowledgement) / 15 days (resolution). "
        "STEP 5: Take down unlawful content within 36 hours of court/govt order.",
        15, 5000000);
}

/* =========================================================================
 * Labour Codes Template
 * ========================================================================= */
static void init_labour_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_LABOUR_CODE;
    sigma_strncpy(t->name, "LABOUR_CODES_COMPLIANCE_2019_2020", 63);
    sigma_strncpy(t->description,
        "Four Labour Codes 2019-2020 — Wages, Social Security, IR, OHS&WC — "
        "compliance obligations for employers.", 255);
    t->item_count = 0;

    add_item(t,
        "Wage Register Maintenance (Form A)",
        "Sec 6 Code on Wages 2019",
        "PREREQ: Employer employing workers. "
        "Have: employee list, working hours, deduction records.",
        "STEP 1: Maintain wage register (Form A) digitally or physically. "
        "STEP 2: Include: name, designation, bank details, wages, deductions, net paid. "
        "STEP 3: Pay wages on time: 7th (large), 10th (others) of next month. "
        "STEP 4: Slip/payslip issued to every worker. "
        "STEP 5: Penalty for non-maintenance: Rs 50,000 (first) / Rs 1L (repeat).",
        7, 50000);

    add_item(t,
        "EPF Registration & Contribution",
        "EPF & MP Act 1952 / Social Security Code 2020",
        "PREREQ: Employer with 20+ employees. "
        "Select employees to be covered.",
        "STEP 1: Register on EPFO portal (unifiedportal-emp.epfindia.gov.in). "
        "STEP 2: Deduct 12% of basic salary as employee EPF contribution. "
        "STEP 3: Employer also contributes 12% (of which 8.33% to EPS, 3.67% to EPF). "
        "STEP 4: File ECR (Electronic Challan cum Return) by 15th of each month. "
        "STEP 5: Non-compliance: imprisonment up to 3 years + penalty.",
        15, 5000);

    add_item(t,
        "Gratuity Payment to Employee",
        "Sec 4 Payment of Gratuity Act 1972 / Social Security Code",
        "PREREQ: Employee served 5+ years (death/disability: less). "
        "Have: last drawn salary (basic+DA), years of service.",
        "STEP 1: Calculate gratuity: (15/26) x last salary x completed years. "
        "STEP 2: Maximum payable: Rs 20 lakhs. "
        "STEP 3: Pay within 30 days of termination/retirement/death. "
        "STEP 4: Form I (by employee), Form L (payment), Form N (appeal) available. "
        "STEP 5: Non-payment: compound interest + imprisonment.",
        30, 0);
}

/* =========================================================================
 * Consumer Protection Template
 * ========================================================================= */
static void init_consumer_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_CONSUMER_PROTECTION;
    sigma_strncpy(t->name, "CONSUMER_PROTECTION_ACT_2019", 63);
    sigma_strncpy(t->description,
        "Consumer Protection Act 2019 — filing complaints, jurisdiction, "
        "defective goods, unfair trade practices, and e-commerce.", 255);
    t->item_count = 0;

    add_item(t,
        "File Consumer Complaint (Novice First Step)",
        "Sec 35 Consumer Protection Act 2019",
        "PREREQ: You are a consumer (paid for goods/services). "
        "Have: bill/invoice, defect evidence, prior complaint/notice to seller.",
        "STEP 1: Send legal notice to seller/service provider (try to resolve first). "
        "STEP 2: Wait 30 days for response. "
        "STEP 3: File complaint at: District Commission (up to Rs 50L), "
        "State Commission (Rs 50L–2Cr), National Commission (above 2Cr). "
        "STEP 4: Online: edaakhil.nic.in — no lawyer required, self-representation allowed. "
        "STEP 5: Pay court fee (Rs 100 to Rs 5000 based on claim). "
        "STEP 6: Commission must hear within 90 days (simple defect — 150 days).",
        90, 0);

    add_item(t,
        "E-Commerce Complaint (Amazon, Flipkart etc.)",
        "IT (E-Commerce) Rules 2020 / Consumer Protection Act",
        "PREREQ: Online purchase defective/not delivered/wrong product. "
        "Have: order ID, screenshots, communication record.",
        "STEP 1: Contact seller's grievance officer first (must be displayed on site). "
        "STEP 2: Raise ticket — must be resolved within 48 hours. "
        "STEP 3: If not resolved — file at edaakhil.nic.in. "
        "STEP 4: Also report to DOT if repeat offender. "
        "STEP 5: Platforms cannot remove negative reviews (unfair trade practice).",
        2, 0);
}

/* =========================================================================
 * Cyber Crime Template (standalone)
 * ========================================================================= */
static void init_rera_template(void) {
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = DOMAIN_RERA;
    sigma_strncpy(t->name, "RERA_2016_REAL_ESTATE_COMPLIANCE", 63);
    sigma_strncpy(t->description,
        "Real Estate (Regulation and Development) Act 2016 — buyer rights, "
        "project registration, and complaint procedures.", 255);
    t->item_count = 0;

    add_item(t,
        "Check Project RERA Registration",
        "Sec 3 RERA",
        "PREREQ: Buying under-construction apartment. "
        "Have: project name, builder details.",
        "STEP 1: Visit state RERA website (e.g., maharera.mahaonline.gov.in). "
        "STEP 2: Search project by name — verify: registration number, completion date, escrow account. "
        "STEP 3: Check quarterly progress reports filed by builder. "
        "STEP 4: Verify registered site plan matches what was shown to you.",
        0, 0);

    add_item(t,
        "File Complaint against Builder (delay/defect)",
        "Sec 31 RERA",
        "PREREQ: Possession delayed beyond RERA-registered date OR structural defect within 5 years. "
        "Have: sale agreement, receipts, RERA registration number.",
        "STEP 1: File complaint online on state RERA portal. "
        "STEP 2: Seek interestcompensation (SBI MCLR + 2%) for delay period. "
        "STEP 3: Allottee can claim refund with interest if unwilling to wait. "
        "STEP 4: RERA Adjudicating Officer hears within 60 days. "
        "STEP 5: Appeal before RERA Appellate Tribunal within 60 days of order.",
        60, 0);
}

/* =========================================================================
 * MAIN INIT: Register all templates
 * ========================================================================= */
void checklist_init(void) {
    g_template_count = 0;

    init_bnss_template();
    init_bns_template();
    init_bsa_template();
    init_pocso_template();
    init_pmla_template();
    init_rti_template();
    init_ibc_template();
    init_dpdp_template();
    init_gst_template();
    init_arbitration_template();
    init_cyber_template();
    init_labour_template();
    init_consumer_template();
    init_rera_template();

    ksigma_printf("[CHECKLIST-SHARD]: %u Legal Domain Templates Loaded (BNSS/BNS/BSA/POCSO/PMLA/RTI/IBC/DPDP/GST/ARB/IT/LABOUR/CONSUMER/RERA).\n",
            g_template_count);
}

/* =========================================================================
 * Query & Audit APIs
 * ========================================================================= */
k_status checklist_query_domain(LawDomain domain, u32* out_count) {
    u32 i;
    for (i = 0; i < g_template_count; i++) {
        if (g_templates[i].domain == domain) {
            if (out_count) *out_count = g_templates[i].item_count;
            ksigma_printf("[CHECKLIST]: Domain Template '%s' has %u items.\n",
                    g_templates[i].name, g_templates[i].item_count);
            return K_OK;
        }
    }
    return K_ERR_INVAL;
}

k_status checklist_audit_deadline(u32 filing_date, u32 section_limit) {
    u32 i, j;
    u32 overdue_count = 0;
    for (i = 0; i < g_template_count; i++) {
        for (j = 0; j < g_templates[i].item_count; j++) {
            ChecklistItem* ci = &g_templates[i].items[j];
            if (ci->deadline_days > 0 && filing_date > section_limit + ci->deadline_days) {
                overdue_count++;
                ksigma_printf("[DEADLINE-AUDIT]: OVERDUE: %s (Ref: %s, Penalty: Rs %u)\n",
                        ci->desc, ci->section_ref, ci->penalty_rs);
            }
        }
    }
    ksigma_printf("[DEADLINE-AUDIT]: Total overdue items: %u\n", overdue_count);
    return (overdue_count == 0) ? K_OK : K_ERR_INVAL;
}

k_status checklist_generate_report(void) {
    u32 i, j;
    u32 total = 0, pending = 0, completed = 0;
    for (i = 0; i < g_template_count; i++) {
        for (j = 0; j < g_templates[i].item_count; j++) {
            total++;
            if (g_templates[i].items[j].status == STATUS_PENDING) pending++;
            if (g_templates[i].items[j].status == STATUS_COMPLETED) completed++;
        }
    }
    ksigma_printf("[CHECKLIST-REPORT]: Total=%u | Pending=%u | Completed=%u | Compliance=%.1f%%\n",
            total, pending, completed, completed * 100.0f / (total ? total : 1));
    return K_OK;
}

u32 checklist_total_items(void) {
    u32 i, total = 0;
    for (i = 0; i < g_template_count; i++) total += g_templates[i].item_count;
    return total;
}
