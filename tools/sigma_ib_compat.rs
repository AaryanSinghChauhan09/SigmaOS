// SPDX-License-Identifier: Apache-2.0
//! SigmaOS Insolvency and Bankruptcy Code (IBC), 2016 Compatibility
//! Corporate Insolvency Resolution Process (CIRP) timeline and CoC voting auditors
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaF64 = f64;

/// Calculate Committee of Creditors (CoC) voting shares
/// Under IBC, voting is based on the proportion of financial debt held by each creditor
/// relative to the total financial debt of the corporate debtor.
#[no_mangle]
pub unsafe extern "C" fn ibc_calculate_voting_share(
    creditor_debt: SigmaF64,
    total_financial_debt: SigmaF64,
) -> SigmaF64 {
    if creditor_debt <= 0.0 || total_financial_debt <= 0.0 || creditor_debt > total_financial_debt {
        return 0.0;
    }
    (creditor_debt / total_financial_debt) * 100.0
}

/// Audit CIRP process timeline thresholds
/// The Corporate Insolvency Resolution Process (CIRP) must be completed within a period of 180 days.
/// It can be extended once, but must be mandatorily completed (including all litigation) within 330 days.
#[no_mangle]
pub unsafe extern "C" fn ibc_audit_cirp_timeline(
    days_elapsed: SigmaU32,
    extension_granted: SigmaBool,
) -> SigmaI32 {
    if days_elapsed > 330 {
        return -2; // Non-compliant: Mandatory max timeline of 330 days exceeded! Must proceed to Liquidation.
    }
    if days_elapsed > 180 {
        if !extension_granted {
            return -1; // Non-compliant: 180 days exceeded without an official extension!
        }
        return 1; // Warning: Project is in extension period (between 180 and 330 days)
    }
    0 // Fully compliant (within the initial 180-day window)
}

/// Verify if a resolution plan has obtained the mandatory approval voting threshold of the CoC
/// Under Section 30(4) of the IBC, a resolution plan requires a minimum of 66% of the voting share
/// of financial creditors to be approved.
#[no_mangle]
pub unsafe extern "C" fn ibc_verify_plan_approval(
    approved_votes_share: SigmaF64,
) -> SigmaBool {
    if approved_votes_share < 0.0 || approved_votes_share > 100.0 {
        return false;
    }
    // Threshold is 66% under current IBC amendments (previously 75%)
    approved_votes_share >= 66.0
}
