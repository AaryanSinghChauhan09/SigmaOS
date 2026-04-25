// SigmaOS — sigma-sec-mac: Mandatory Access Control
// Inspired by: SELinux, AppArmor, SMACK
// Module: sigma-sec-mac
// USP: No kernel LSM hooks needed — pure userspace policy table
// Policy: subject (process) × object (resource) × permission = allow/deny

#ifndef SIGMA_SEC_MAC_H
#define SIGMA_SEC_MAC_H

#define SIGMA_MAC_MAX_LABELS   32
#define SIGMA_MAC_MAX_RULES   128
#define SIGMA_MAC_LABEL_LEN    24

#define SIGMA_MAC_ALLOW  1
#define SIGMA_MAC_DENY   0

// Permissions bitmask
#define SIGMA_MAC_PERM_READ    0x01
#define SIGMA_MAC_PERM_WRITE   0x02
#define SIGMA_MAC_PERM_EXEC    0x04
#define SIGMA_MAC_PERM_NET     0x08
#define SIGMA_MAC_PERM_IPC     0x10
#define SIGMA_MAC_PERM_DEVICE  0x20

typedef struct SigmaMACLabel {
    char         name[SIGMA_MAC_LABEL_LEN];
    unsigned int label_id;
} SigmaMACLabel;

typedef struct SigmaMACRule {
    unsigned int subject_id;    // process label
    unsigned int object_id;     // resource label
    unsigned char permissions;  // bitmask
    unsigned char verdict;      // ALLOW or DENY
    unsigned long hit_count;
} SigmaMACRule;

typedef struct SigmaMACPolicy {
    SigmaMACLabel  labels[SIGMA_MAC_MAX_LABELS];
    SigmaMACRule   rules[SIGMA_MAC_MAX_RULES];
    unsigned int   label_count;
    unsigned int   rule_count;
    unsigned char  default_verdict; // ALLOW or DENY (fail-closed = DENY)
} SigmaMACPolicy;

static inline void mac_init(SigmaMACPolicy* p, unsigned char default_v) {
    p->label_count = p->rule_count = 0;
    p->default_verdict = default_v;
}

static inline unsigned int mac_add_label(SigmaMACPolicy* p, const char* name) {
    if (p->label_count >= SIGMA_MAC_MAX_LABELS) return 0xFFFFFFFF;
    SigmaMACLabel* l = &p->labels[p->label_count];
    for (int i = 0; i < SIGMA_MAC_LABEL_LEN - 1 && name[i]; i++) l->name[i] = name[i];
    l->label_id = p->label_count++;
    return l->label_id;
}

static inline int mac_add_rule(SigmaMACPolicy* p,
                                unsigned int subject, unsigned int object,
                                unsigned char perms, unsigned char verdict) {
    if (p->rule_count >= SIGMA_MAC_MAX_RULES) return -1;
    SigmaMACRule* r = &p->rules[p->rule_count++];
    r->subject_id  = subject; r->object_id = object;
    r->permissions = perms;   r->verdict   = verdict;
    r->hit_count   = 0;
    return (int)(p->rule_count - 1);
}

// Check access: returns ALLOW(1) or DENY(0)
static inline int mac_check(SigmaMACPolicy* p,
                              unsigned int subject, unsigned int object,
                              unsigned char requested_perm) {
    for (unsigned int i = 0; i < p->rule_count; i++) {
        SigmaMACRule* r = &p->rules[i];
        if (r->subject_id != subject || r->object_id != object) continue;
        if (!(r->permissions & requested_perm)) continue;
        r->hit_count++;
        return r->verdict;
    }
    return p->default_verdict;
}

static inline unsigned long mac_total_denials(SigmaMACPolicy* p) {
    unsigned long n = 0;
    for (unsigned int i = 0; i < p->rule_count; i++)
        if (p->rules[i].verdict == SIGMA_MAC_DENY) n += p->rules[i].hit_count;
    return n;
}

#endif /* SIGMA_SEC_MAC_H */
