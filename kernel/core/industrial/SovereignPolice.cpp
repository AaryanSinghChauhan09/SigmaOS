#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Indian Police / IPS Shard (S-POLICE)
 * Purpose: Operational tools for Indian police officers and investigators.
 * Standards: BNS 2023 (formerly IPC), BNSS 2023 (formerly CrPC), IEA 2023 (Evidence Act),
 *            NIA Act 2008, NDPS Act 1985, Prevention of Corruption Act 1988.
 * Features: FIR triager, Remand calculator, PC Act penalty lookup, NDPS schedule classifier.
 */

namespace SigmaOS {
namespace Kernel {
namespace Legal {

// NDPS Act 1985 quantity thresholds (commercial = mandatory minimum 10 years)
struct NDPSEntry {
    const char* substance;
    sigma_u32 small_qty_g;   // Small quantity (grams)
    sigma_u32 commercial_g;  // Commercial quantity (grams)
};

static const NDPSEntry NDPS_TABLE[] = {
    {"Cannabis_Ganja",   1000,  20000},
    {"Hashish_Charas",     100,   1000},
    {"Heroin",               5,    250},
    {"Cocaine",              2,    100},
    {"Methamphetamine",      5,     50},
    {"Opium",               25,   2500},
    {"Morphine",             5,    250},
};
static const sigma_u32 NDPS_LEN = sizeof(NDPS_TABLE) / sizeof(NDPS_TABLE[0]);

class SovereignPolice : public SigmaOS::SigmaObject {
public:
    static SovereignPolice& getInstance() {
        static SovereignPolice instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignPolice"; }

    void init() {
        sigma_log_info("[S-POLICE] Initializing Indian Law Enforcement Nexus...");
        sigma_log_info("[S-POLICE] Laws: BNS 2023 | BNSS 2023 | IEA 2023 | NDPS 1985 | PC Act 1988");
    }

    /**
     * FIR category triager based on cognizability and bailable nature.
     * Cognizable if: punishable by ≥3 years imprisonment (Sec 2(c) BNSS).
     * @param max_sentence_years  Maximum sentence for the alleged offence
     * @param bailable            Whether offence is bailable
     */
    void triageFIR(sigma_u32 max_sentence_years, bool bailable) {
        bool cognizable = (max_sentence_years >= 3);
        sigma_log_info("[S-POLICE] FIR Triage | Max sentence: %u yr | Cognizable: %s | Bailable: %s",
                       max_sentence_years,
                       cognizable ? "YES (arrest without warrant)" : "NO (warrant required)",
                       bailable   ? "YES"                          : "NO");
        if (cognizable) {
            sigma_log_info("[S-POLICE] → File FIR u/s Sec 173 BNSS immediately. IO to investigate.");
        } else {
            sigma_log_info("[S-POLICE] → Non-cognizable report (NCR). Seek Magistrate permission u/s 175 BNSS.");
        }
    }

    /**
     * BNSS 2023 Remand calculator (Sec 187 / 167 CrPC equivalent).
     * Police remand: max 15 days total (extendable to 60/90 days in judicial custody).
     * @param offence_max_years  Max sentence for offence
     * @param days_in_custody    Days already in custody
     */
    void calcRemand(sigma_u32 offence_max_years, sigma_u32 days_in_custody) {
        sigma_u32 max_jc = (offence_max_years >= 10) ? 90 : 60; // 90-day JC for ≥10yr offences
        sigma_u32 police_rem_remaining = (days_in_custody < 15) ? 15 - days_in_custody : 0;
        sigma_u32 jc_remaining = (days_in_custody < max_jc) ? max_jc - days_in_custody : 0;

        sigma_log_info("[S-POLICE] BNSS Remand | Day %u | Police remand remaining: %u days | JC limit: %u days",
                       days_in_custody, police_rem_remaining, max_jc);
        sigma_log_info("[S-POLICE]   Max JC remaining: %u days before mandatory bail/chargesheet", jc_remaining);
        sigma_log_info("[S-POLICE]   Chargesheet deadline: Day 60/90 per Sec 187 BNSS.");
    }

    /**
     * NDPS Act 1985 quantity classifier.
     * @param substance   e.g. "Heroin"
     * @param qty_grams   Seized quantity in grams
     */
    void ndpsClassify(const char* substance, sigma_u32 qty_grams) {
        for (sigma_u32 i = 0; i < NDPS_LEN; ++i) {
            bool m = true;
            for (sigma_u32 j = 0; NDPS_TABLE[i].substance[j] || substance[j]; ++j) {
                if (NDPS_TABLE[i].substance[j] != substance[j]) { m = false; break; }
            }
            if (!m) continue;
            const char* cat;
            if (qty_grams < NDPS_TABLE[i].small_qty_g)
                cat = "SMALL QUANTITY (max 1 yr) — Sec 27 NDPS";
            else if (qty_grams < NDPS_TABLE[i].commercial_g)
                cat = "INTERMEDIATE (max 10 yr) — Sec 20/21/22 NDPS";
            else
                cat = "COMMERCIAL QUANTITY (min 10–20 yr) — Sec 20/21/22 NDPS";
            sigma_log_info("[S-POLICE] NDPS | %s | Qty: %u g | Category: %s",
                           substance, qty_grams, cat);
            return;
        }
        sigma_log_err("[S-POLICE] Substance '%s' not in NDPS table.", substance);
    }

    /**
     * Prevention of Corruption Act 1988 (PC Act) — penalty section lookup.
     */
    void pcActLookup(sigma_u32 offence_code) {
        switch (offence_code) {
            case 7:  sigma_log_info("[S-POLICE] PC Act Sec 7: Taking gratification (3-7 yr + fine)."); break;
            case 11: sigma_log_info("[S-POLICE] PC Act Sec 11: Obtaining valuable things w/o consideration (3-7 yr)."); break;
            case 13: sigma_log_info("[S-POLICE] PC Act Sec 13: Criminal misconduct by public servant (4-10 yr)."); break;
            case 14: sigma_log_info("[S-POLICE] PC Act Sec 14: Abetment of offences (same as principal)."); break;
            default: sigma_log_err("[S-POLICE] PC Act section %u not in quick-reference.", offence_code);
        }
    }
};

} // namespace Legal
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void police_init() {
    SigmaOS::Kernel::Legal::SovereignPolice::getInstance().init();
}

void police_fir(sigma_u32 max_yr, bool bailable) {
    SigmaOS::Kernel::Legal::SovereignPolice::getInstance().triageFIR(max_yr, bailable);
}

void police_remand(sigma_u32 max_yr, sigma_u32 days) {
    SigmaOS::Kernel::Legal::SovereignPolice::getInstance().calcRemand(max_yr, days);
}

void police_ndps(const char* sub, sigma_u32 qty_g) {
    SigmaOS::Kernel::Legal::SovereignPolice::getInstance().ndpsClassify(sub, qty_g);
}

void police_pc_act(sigma_u32 sec) {
    SigmaOS::Kernel::Legal::SovereignPolice::getInstance().pcActLookup(sec);
}

} // extern "C"
 