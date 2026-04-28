/* =========================================================================
 * Σ SIGMAOS: SOVEREIGN LEGAL ENGINE SHARD (v3.1 - INDUSTRIAL FINALITY)
 * =========================================================================
 * Mission: Sharding judicial compliance and criminal procedure.
 * Principles: Zero-Dependency, Silicon-Direct, Rule-of-Law-Native.
 * =========================================================================
 */

#include "../../include/sigma_types.h"
#include "../../include/SovereignLibC.h"

#define MAX_CHECKLIST_ITEMS 32
#define MAX_TEMPLATES      16
#define MAX_DESC_LEN       256
#define MAX_REF_LEN        64
#define MAX_STEP_LEN       512
#define MAX_PREREQ_LEN     128

typedef enum {
    DOMAIN_CRIMINAL_BNSS,
    DOMAIN_SUBSTANTIVE_BNS,
    DOMAIN_EVIDENCE_BSA,
    DOMAIN_PMLA,
    DOMAIN_DPDP_ACT,
    DOMAIN_CYBER_IT_ACT,
    DOMAIN_RTI,
    DOMAIN_INSOLVENCY_IBC,
    DOMAIN_TAX_GST,
    DOMAIN_POCSO,
    DOMAIN_ARBITRATION,
    DOMAIN_LABOUR_CODE,
    DOMAIN_CONSUMER_PROTECTION,
    DOMAIN_RERA
} LawDomain;

typedef enum {
    STATUS_PENDING,
    STATUS_COMPLETED,
    STATUS_OVERDUE
} ItemStatus;

typedef struct {
    char desc[MAX_DESC_LEN];
    char section_ref[MAX_REF_LEN];
    char prerequisites[MAX_PREREQ_LEN];
    char procedure[MAX_STEP_LEN];
    sigma_u32 deadline_days;
    sigma_u32 penalty_rs;
    ItemStatus status;
} ChecklistItem;

typedef struct {
    LawDomain domain;
    char name[64];
    char description[256];
    ChecklistItem items[MAX_CHECKLIST_ITEMS];
    sigma_u32 item_count;
} LegalTemplate;

static LegalTemplate g_templates[MAX_TEMPLATES];
static sigma_u32 g_template_count = 0;

/* =========================================================================
 * Core Template API
 * ========================================================================= */
static void add_item(void* t_ptr,
                      const char* desc,
                      const char* ref,
                      const char* prereq,
                      const char* proc,
                      sigma_u32 days,
                      sigma_u32 penalty_rs) {
    LegalTemplate* t = (LegalTemplate*)t_ptr;
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

static void create_template(LawDomain domain, const char* name, const char* desc, void (*init_fn)(void*)) {
    if (g_template_count >= MAX_TEMPLATES) return;
    LegalTemplate* t = &g_templates[g_template_count++];
    t->domain = domain;
    sigma_strncpy(t->name, name, 63);
    sigma_strncpy(t->description, desc, 255);
    t->item_count = 0;
    init_fn(t);
}

/* =========================================================================
 * Template Definitions (BNSS, BNS, BSA)
 * ========================================================================= */
void init_bnss_template(void* t_ptr) {
    LegalTemplate* t = (LegalTemplate*)t_ptr;
    add_item(t, "Zero FIR Filing", "Sec 173 BNSS", "Incident out of station.", "File at any PS.", 0, 0);
}

void init_bns_template(void* t_ptr) {
    LegalTemplate* t = (LegalTemplate*)t_ptr;
    add_item(t, "Self Defence Shard", "Sec 34-44 BNS", "Threat to life/property.", "Use proportional force.", 0, 0);
}

void init_bsa_template(void* t_ptr) {
    LegalTemplate* t = (LegalTemplate*)t_ptr;
    add_item(t, "Electronic Evidence Certification", "Sec 63 BSA", "Digital records.", "Submit 65B certificate.", 0, 0);
}

void init_pmla_template(void* t) { (void)t; }
void init_dpdp_template(void* t) { (void)t; }
void init_cyber_template(void* t) { (void)t; }
void init_rti_template(void* t) { (void)t; }
void init_ibc_template(void* t) { (void)t; }
void init_gst_template(void* t) { (void)t; }
void init_pocso_template(void* t) { (void)t; }
void init_arbitration_template(void* t) { (void)t; }
void init_labour_template(void* t) { (void)t; }
void init_consumer_template(void* t) { (void)t; }
void init_rera_template(void* t) { (void)t; }

/* =========================================================================
 * Main Init
 * ========================================================================= */
void checklist_init(void) {
    g_template_count = 0;
    create_template(DOMAIN_CRIMINAL_BNSS, "BNSS_2023", "Criminal Procedure", init_bnss_template);
    create_template(DOMAIN_SUBSTANTIVE_BNS, "BNS_2023", "Substantive Offences", init_bns_template);
    create_template(DOMAIN_EVIDENCE_BSA, "BSA_2023", "Evidence & Forensics", init_bsa_template);
    
    sigma_printf("[CHECKLIST-ZENITH]: Legal Engine Online. BNSS/BNS/BSA Shards Active.\n");
}
