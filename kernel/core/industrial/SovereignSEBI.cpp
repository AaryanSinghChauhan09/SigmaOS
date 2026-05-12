#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign SEBI / Stock Market Shard (S-SEBI)
 * Purpose: Investment compliance & financial analytics for Indian stock brokers, analysts, and investors.
 * Standards: SEBI Act 1992, SEBI (LODR) 2015, SEBI (PIT) 2015, PMLA 2002,
 *            NSE/BSE F&O margin rules, SEBI circular on SLB, T+1 settlement.
 * Features: CAGR, SIP return, PE/PB ratio check, F&O margin calculator, Insider Trading flag.
 */

namespace SigmaOS {
namespace Kernel {
namespace Finance {

// SEBI-mandated upfront margin rates for F&O (SPAN + Exposure)
struct FnOMargin {
    const char* instrument;
    sigma_u32 span_permille;     // SPAN margin %
    sigma_u32 exposure_permille; // Exposure margin %
};

static const FnOMargin FNO_MARGIN_TABLE[] = {
    {"NIFTY_FUT",   90,  20},   // ~9% SPAN + 2% exposure
    {"BANKNIFTY_FUT", 90, 30},
    {"SENSEX_FUT",   90, 20},
    {"STOCK_FUT",   140, 50},   // ~14% SPAN + 5% exposure (typical)
    {"INDEX_OPT",   30,  10},   // Buyer pays only premium
    {"STOCK_OPT",   30,  20},
};
static const sigma_u32 FNO_LEN = sizeof(FNO_MARGIN_TABLE) / sizeof(FNO_MARGIN_TABLE[0]);

class SovereignSEBI : public SigmaOS::SigmaObject {
public:
    static SovereignSEBI& getInstance() {
        static SovereignSEBI instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignSEBI"; }

    void init() {
        sigma_log_info("[S-SEBI] Initializing Indian Capital Markets Nexus...");
        sigma_log_info("[S-SEBI] Standards: SEBI Act 1992 | LODR 2015 | PIT 2015 | PMLA 2002 | T+1 Settlement");
    }

    /**
     * CAGR calculator.
     * CAGR = (Ending / Beginning)^(1/years) - 1
     * Uses integer approximation via Newton's method (n-th root).
     * @param begin_paise   Initial investment in paise
     * @param end_paise     Final value in paise
     * @param years         Investment tenure in years
     */
    void calcCAGR(sigma_u64 begin_paise, sigma_u64 end_paise, sigma_u32 years) {
        if (begin_paise == 0 || years == 0) {
            sigma_log_err("[S-SEBI] Invalid CAGR parameters.");
            return;
        }
        // ratio_x1000 = end * 1000 / begin
        sigma_u64 ratio_x1000 = (end_paise * 1000ULL) / begin_paise;
        // n-th root via integer successive approximation (Newton)
        sigma_u64 root = ratio_x1000;
        for (sigma_u32 i = 0; i < 40; ++i) {
            // x_{n+1} = ((years-1)*x_n + ratio/x_n^(years-1)) / years
            // Simplified: one-iteration Newton for square-cube roots; for larger years iterate
            sigma_u64 prev = root;
            // power of (years-1) — use 64-bit
            sigma_u64 pw = 1000ULL;
            for (sigma_u32 j = 1; j < years; ++j) pw = (pw * prev) / 1000ULL;
            if (pw == 0) break;
            root = ((sigma_u64)(years - 1) * prev + ratio_x1000 * 1000000ULL / pw) / years;
            if (root == prev) break;
        }
        sigma_u64 cagr_permille = root - 1000ULL; // (root - 1) * 1000
        sigma_log_info("[S-SEBI] CAGR | Begin: ₹%llu | End: ₹%llu | Years: %u | CAGR: %llu.%01llu%%",
                       begin_paise/100, end_paise/100, years,
                       cagr_permille/10, cagr_permille%10);
    }

    /**
     * SIP future value calculator (monthly compounding).
     * FV = P * [(1+r)^n - 1] / r * (1+r)
     * @param monthly_sip_paise  Monthly SIP amount in paise
     * @param annual_rate_bp     Annual expected return in basis points (e.g. 1200 = 12%)
     * @param months             Investment tenure in months
     */
    void calcSIP(sigma_u64 monthly_sip_paise, sigma_u32 annual_rate_bp, sigma_u32 months) {
        if (months == 0 || annual_rate_bp == 0) {
            sigma_log_err("[S-SEBI] Invalid SIP parameters.");
            return;
        }
        // Monthly rate r_micro = annual_bp * 10^6 / (12 * 10000)
        sigma_u64 r_micro = (sigma_u64)annual_rate_bp * 1000000ULL / 120000ULL;
        sigma_u64 base    = 1000000ULL + r_micro;

        // (1+r)^n
        sigma_u64 pow_m = 1000000ULL;
        for (sigma_u32 i = 0; i < months; ++i)
            pow_m = (pow_m * base) / 1000000ULL;

        // FV = P * (pow_m - 1) / r_micro * (1+r)
        if (r_micro == 0) { sigma_log_err("[S-SEBI] Rate too low."); return; }
        sigma_u64 fv = (monthly_sip_paise * (pow_m - 1000000ULL) / r_micro) * base / 1000000ULL;
        sigma_u64 invested = monthly_sip_paise * months;
        sigma_u64 gain     = (fv > invested) ? fv - invested : 0;

        sigma_log_info("[S-SEBI] SIP | Monthly: ₹%llu | %u mo | Rate: %u.%02u%% p.a. | FV: ₹%llu | Gain: ₹%llu",
                       monthly_sip_paise/100, months,
                       annual_rate_bp/100, annual_rate_bp%100,
                       fv/100, gain/100);
    }

    /**
     * F&O margin requirement lookup.
     * @param instrument  e.g. "NIFTY_FUT"
     * @param lot_value   Lot value in paise (contract price * lot size)
     */
    void calcFnOMargin(const char* instrument, sigma_u64 lot_value_paise) {
        for (sigma_u32 i = 0; i < FNO_LEN; ++i) {
            bool m = true;
            for (sigma_u32 j = 0; FNO_MARGIN_TABLE[i].instrument[j] || instrument[j]; ++j) {
                if (FNO_MARGIN_TABLE[i].instrument[j] != instrument[j]) { m = false; break; }
            }
            if (!m) continue;
            sigma_u64 span = (lot_value_paise * FNO_MARGIN_TABLE[i].span_permille) / 1000ULL;
            sigma_u64 exp  = (lot_value_paise * FNO_MARGIN_TABLE[i].exposure_permille) / 1000ULL;
            sigma_log_info("[S-SEBI] F&O Margin | %s | Lot: ₹%llu | SPAN: ₹%llu | Exposure: ₹%llu | Total: ₹%llu",
                           instrument, lot_value_paise/100, span/100, exp/100, (span+exp)/100);
            return;
        }
        sigma_log_err("[S-SEBI] Instrument '%s' not in margin table.", instrument);
    }

    /**
     * SEBI PIT Regulation 2015 — Insider Trading flag.
     * Flags if trade is within 2-day or 30-day window before results announcement.
     * @param days_to_results  Days until quarterly results announcement
     */
    void checkInsiderWindow(sigma_u32 days_to_results) {
        if (days_to_results <= 2) {
            sigma_log_info("[S-SEBI] 🚨 PIT REGULATION: Trade within 2-day BLACKOUT WINDOW — PROHIBITED.");
        } else if (days_to_results <= 30) {
            sigma_log_info("[S-SEBI] ⚠️  PIT REGULATION: Within 30-day trading window — report if UPSI held.");
        } else {
            sigma_log_info("[S-SEBI] ✅ PIT: Outside restricted window — trade PERMITTED.");
        }
    }
};

} // namespace Finance
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sebi_init() {
    SigmaOS::Kernel::Finance::SovereignSEBI::getInstance().init();
}

void sebi_cagr(sigma_u64 begin, sigma_u64 end, sigma_u32 years) {
    SigmaOS::Kernel::Finance::SovereignSEBI::getInstance().calcCAGR(begin, end, years);
}

void sebi_sip(sigma_u64 monthly, sigma_u32 rate_bp, sigma_u32 months) {
    SigmaOS::Kernel::Finance::SovereignSEBI::getInstance().calcSIP(monthly, rate_bp, months);
}

void sebi_fno_margin(const char* instr, sigma_u64 lot_val) {
    SigmaOS::Kernel::Finance::SovereignSEBI::getInstance().calcFnOMargin(instr, lot_val);
}

void sebi_pit(sigma_u32 days) {
    SigmaOS::Kernel::Finance::SovereignSEBI::getInstance().checkInsiderWindow(days);
}

} // extern "C"
