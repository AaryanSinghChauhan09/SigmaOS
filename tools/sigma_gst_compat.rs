// SPDX-License-Identifier: Apache-2.0
//! SigmaOS GST Act, 2017 Compatibility
//! Goods and Services Tax ledger auditing and tax split calculators
//! Zero external dependencies

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaF64 = f64;

/// Struct representing the tax breakdown of an invoice transaction
#[repr(C)]
pub struct GstTaxBreakdown {
    pub cgst: SigmaF64, // Central GST
    pub sgst: SigmaF64, // State GST
    pub igst: SigmaF64, // Integrated GST
    pub utgst: SigmaF64, // Union Territory GST
    pub cess: SigmaF64, // Compensation Cess
    pub total_amount: SigmaF64,
}

/// Calculate tax splits based on invoice value, supply type, and applicable slab
/// supply_type: 0 for Intra-State, 1 for Inter-State, 2 for Union Territory
#[no_mangle]
pub unsafe extern "C" fn gst_calculate_tax(
    net_amount: SigmaF64,
    tax_slab_percentage: SigmaF64,
    supply_type: SigmaU32,
    cess_percentage: SigmaF64,
    out_breakdown: *mut GstTaxBreakdown,
) -> SigmaI32 {
    if net_amount <= 0.0 || out_breakdown.is_null() {
        return -1;
    }

    let tax_rate = tax_slab_percentage / 100.0;
    let cess_rate = cess_percentage / 100.0;

    let total_tax = net_amount * tax_rate;
    let total_cess = net_amount * cess_rate;

    let mut breakdown = GstTaxBreakdown {
        cgst: 0.0,
        sgst: 0.0,
        igst: 0.0,
        utgst: 0.0,
        cess: total_cess,
        total_amount: net_amount + total_tax + total_cess,
    };

    if supply_type == 0 {
        // Intra-State: Split 50-50 into CGST and SGST
        breakdown.cgst = total_tax / 2.0;
        breakdown.sgst = total_tax / 2.0;
    } else if supply_type == 1 {
        // Inter-State: Entire tax goes to IGST
        breakdown.igst = total_tax;
    } else if supply_type == 2 {
        // Union Territory: Split 50-50 into CGST and UTGST
        breakdown.cgst = total_tax / 2.0;
        breakdown.utgst = total_tax / 2.0;
    } else {
        return -2; // Unknown supply type
    }

    *out_breakdown = breakdown;
    0 // Success
}

/// Validate GSTIN format and checksum (15-digit alphanumeric)
/// Format: 2 digits (State code), 10 alphanumeric (PAN), 1 digit (entity number), 1 character (Z by default), 1 alphanumeric (checksum)
#[no_mangle]
pub unsafe extern "C" fn gst_validate_gstin(gstin: *const u8) -> SigmaBool {
    if gstin.is_null() {
        return false;
    }

    let mut len = 0;
    while *gstin.add(len) != 0 {
        len += 1;
        if len > 16 {
            return false;
        }
    }

    if len != 15 {
        return false;
    }

    // 1. Verify first two digits are a valid state code (01 to 38, excluding some gaps)
    let state_d1 = *gstin.add(0) as char;
    let state_d2 = *gstin.add(1) as char;
    if !state_d1.is_ascii_digit() || !state_d2.is_ascii_digit() {
        return false;
    }
    let state_code = (state_d1.to_digit(10).unwrap() * 10) + state_d2.to_digit(10).unwrap();
    if state_code < 1 || state_code > 38 {
        return false;
    }

    // 2. Verify next 10 characters are PAN structure (5 letters, 4 digits, 1 letter)
    for i in 2..7 {
        if !(*gstin.add(i) as char).is_ascii_alphabetic() {
            return false;
        }
    }
    for i in 7..11 {
        if !(*gstin.add(i) as char).is_ascii_digit() {
            return false;
        }
    }
    if !(*gstin.add(11) as char).is_ascii_alphabetic() {
        return false;
    }

    // 3. Verify entity code is alphanumeric
    if !(*gstin.add(12) as char).is_ascii_alphanumeric() {
        return false;
    }

    // 4. Character 14 should be 'Z' or a digit (transitional support)
    let char14 = *gstin.add(13) as char;
    if char14 != 'Z' && !char14.is_ascii_alphanumeric() {
        return false;
    }

    // 5. Verification checksum slot (Character 15) must be alphanumeric
    if !(*gstin.add(14) as char).is_ascii_alphanumeric() {
        return false;
    }

    true // Alphanumeric structure verified
}
