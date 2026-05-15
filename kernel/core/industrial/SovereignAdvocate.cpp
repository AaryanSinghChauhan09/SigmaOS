#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/core/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Indian Advocate Shard (S-ADVOCATE)
 * Purpose: Legal research & compliance tools for Indian advocates, judges, and paralegals.
 * Standards: IPC 1860 / BNS 2023, CrPC 1973 / BNSS 2023, CPC 1908, Limitation Act 1963,
 *            Consumer Protection Act 2019, RTI Act 2005, POCSO Act 2012.
 * Features: Limitation period calculator, bail eligibility, RTI response deadline tracker,
 *           IPC↔BNS section mapper.
 */

namespace SigmaOS {
namespace Kernel {
namespace Legal {

// Limitation periods per Limitation Act 1963 Schedule
struct LimitationEntry {
    sigma_u32 article;
    sigma_u32 period_years;
    const char* description;
};

static const LimitationEntry LIMITATION_TABLE[] = {
    {  1,  3, "Suit on account of money"},
    { 55,  3, "Suit for recovery of money on bond"},
    { 58,  3, "Suit for property through executor"},
    { 64,  3, "Suit for trespass on immovable property"},
    {113,  3, "Civil suit (residual category)"},
    {116,  3, "Appeal in civil court"},
    {117,  3, "Appeal to High Court"},
    { 36, 12, "Suit for possession of immovable property"},
    { 65, 12, "Adverse possession claim"},
};
static const sigma_u32 LIM_TABLE_LEN = sizeof(LIMITATION_TABLE) / sizeof(LIMITATION_TABLE[0]);

// IPC → BNS 2023 quick mapping (selected key sections)
struct IPCBNSMap {
    sigma_u32 ipc;
    sigma_u32 bns;
    const char* offence;
};

static const IPCBNSMap IPC_BNS[] = {
    {302, 101, "Murder"},
    {304,  99, "Culpable homicide not amounting to murder"},
    {307, 109, "Attempt to murder"},
    {376, 63,  "Rape"},
    {379, 303, "Theft"},
    {380, 305, "Theft in dwelling house"},
    {395, 310, "Dacoity"},
    {420, 318, "Cheating and dishonestly inducing delivery"},
    {498, 84,  "Cruelty by husband or relatives"},
    {120, 61,  "Criminal conspiracy"},
    {500, 356, "Defamation"},
    {323, 115, "Voluntarily causing hurt"},
    {406, 316, "Criminal breach of trust"},
};
static const sigma_u32 IPC_BNS_LEN = sizeof(IPC_BNS) / sizeof(IPC_BNS[0]);

class SovereignAdvocate : public SigmaOS::SigmaObject {
public:
    static SovereignAdvocate& getInstance() {
        static SovereignAdvocate instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignAdvocate"; }

    void init() {
        sigma_log_info("[S-ADVOCATE] Initializing Indian Legal Research Nexus...");
        sigma_log_info("[S-ADVOCATE] Laws: BNS 2023 | BNSS 2023 | CPC 1908 | Limitation Act 1963");
    }

    /**
     * Map an IPC section to its BNS 2023 equivalent.
     */
    void mapIPCtoBNS(sigma_u32 ipc_section) {
        for (sigma_u32 i = 0; i < IPC_BNS_LEN; ++i) {
            if (IPC_BNS[i].ipc == ipc_section) {
                sigma_log_info("[S-ADVOCATE] IPC §%u → BNS §%u | Offence: %s",
                               ipc_section, IPC_BNS[i].bns, IPC_BNS[i].offence);
                return;
            }
        }
        sigma_log_info("[S-ADVOCATE] IPC §%u not in quick-map — consult BNS 2023 directly.", ipc_section);
    }

    /**
     * Limitation period lookup by article number.
     */
    void lookupLimitation(sigma_u32 article) {
        for (sigma_u32 i = 0; i < LIM_TABLE_LEN; ++i) {
            if (LIMITATION_TABLE[i].article == article) {
                sigma_log_info("[S-ADVOCATE] Limitation Act 1963, Art %u: %u year(s) | %s",
                               article, LIMITATION_TABLE[i].period_years,
                               LIMITATION_TABLE[i].description);
                return;
            }
        }
        sigma_log_err("[S-ADVOCATE] Article %u not found.", article);
    }

    /**
     * RTI response deadline: 30 days (general), 48 hours (life/liberty per Sec 7(1)).
     * @param filed_day  Day of month filed (1-28)
     * @param urgent     true if involves life or liberty
     */
    void rtiDeadline(sigma_u32 filed_day, bool urgent) {
        sigma_u32 deadline_days = urgent ? 2 : 30;
        sigma_u32 due_day = filed_day + deadline_days;
        sigma_log_info("[S-ADVOCATE] RTI Act 2005 | Filed: Day %u | Deadline: Day %u (±month rollover) | Type: %s",
                       filed_day, due_day, urgent ? "URGENT (Sec 7(1) — life/liberty)" : "Standard (30 days)");
    }

    /**
     * Bail eligibility quick-check under BNSS 2023.
     * Non-bailable if: offence carries ≥7 years imprisonment.
     */
    void bailCheck(sigma_u32 max_sentence_years, bool repeat_offender) {
        bool bailable = (max_sentence_years < 7) && !repeat_offender;
        sigma_log_info("[S-ADVOCATE] BNSS 2023 Bail Check | Max sentence: %u yr | Repeat: %s | Result: %s",
                       max_sentence_years, repeat_offender ? "Yes" : "No",
                       bailable ? "BAILABLE (Sec 479 BNSS)" : "NON-BAILABLE — court discretion");
    }

    /**
     * Consumer complaint jurisdiction (CP Act 2019).
     * District: up to ₹50L | State: 50L–₹2Cr | National: >₹2Cr
     */
    void consumerForum(sigma_u64 claim_paise) {
        sigma_u64 l50  = 5000000ULL * 100;    // ₹50 lakh
        sigma_u64 cr2  = 200000000ULL * 100;   // ₹2 crore
        const char* forum = (claim_paise <= l50) ? "District Commission (≤₹50 lakh)"
                          : (claim_paise <= cr2) ? "State Commission (₹50L–₹2Cr)"
                                                 : "National Commission (>₹2 Cr)";
        sigma_log_info("[S-ADVOCATE] CP Act 2019 | Claim: ₹%llu | Forum: %s",
                       claim_paise / 100, forum);
    }
};

} // namespace Legal
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void advocate_init() {
    SigmaOS::Kernel::Legal::SovereignAdvocate::getInstance().init();
}

void advocate_ipc_bns(sigma_u32 ipc_sec) {
    SigmaOS::Kernel::Legal::SovereignAdvocate::getInstance().mapIPCtoBNS(ipc_sec);
}

void advocate_limitation(sigma_u32 article) {
    SigmaOS::Kernel::Legal::SovereignAdvocate::getInstance().lookupLimitation(article);
}

void advocate_rti(sigma_u32 day, bool urgent) {
    SigmaOS::Kernel::Legal::SovereignAdvocate::getInstance().rtiDeadline(day, urgent);
}

void advocate_bail(sigma_u32 max_yr, bool repeat) {
    SigmaOS::Kernel::Legal::SovereignAdvocate::getInstance().bailCheck(max_yr, repeat);
}

void advocate_consumer(sigma_u64 claim_paise) {
    SigmaOS::Kernel::Legal::SovereignAdvocate::getInstance().consumerForum(claim_paise);
}

} // extern "C"
