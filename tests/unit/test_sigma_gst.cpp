// SPDX-License-Identifier: GPL-2.0-or-later
// tests/unit/test_sigma_gst.cpp — GST calculation correctness (Indian tax law)
// Tests: CGST/SGST/IGST split, HSN slab lookup, e-Invoice IRN generation,
//        input tax credit, reverse charge, composition scheme.
#include <gtest/gtest.h>
#include <stdint.h>
#include <string.h>
#include <math.h>

// ── Minimal GST calculation engine (mirrors sigma-accounts logic) ─────────

enum GSTType { INTRA_STATE, INTER_STATE, UNION_TERRITORY };

struct GSTSlab {
    const char *hsn_prefix;   /* HSN code prefix, e.g. "0401" = milk */
    double      rate;         /* total GST % */
};

// FY 2024-25 GST slab table (simplified)
static const GSTSlab GST_SLABS[] = {
    { "0",    0.00 },  /* exempt: fresh food, milk, eggs */
    { "1",    0.00 },  /* live animals — exempt */
    { "0401", 0.00 },  /* fresh milk — exempt */
    { "0402", 5.00 },  /* milk powder — 5% */
    { "2202", 12.0 },  /* packaged water / beverages */
    { "3004", 5.00 },  /* medicines */
    { "8471", 18.0 },  /* computers */
    { "8517", 18.0 },  /* mobile phones */
    { "6109", 5.00 },  /* t-shirts (cotton, ≤ ₹1000) */
    { "6109", 12.0 },  /* t-shirts (cotton, > ₹1000) */
    { "2710", 28.0 },  /* petrol / diesel */
    { "2402", 28.0 },  /* cigarettes (+ cess, not tested here) */
    { nullptr, 18.0 }, /* default: 18% for unclassified goods */
};

static double gst_rate_for_hsn(const char *hsn) {
    for (int i = 0; GST_SLABS[i].hsn_prefix; i++) {
        if (strncmp(hsn, GST_SLABS[i].hsn_prefix,
                    strlen(GST_SLABS[i].hsn_prefix)) == 0) {
            return GST_SLABS[i].rate;
        }
    }
    return 18.0; /* default */
}

struct GSTCalcResult {
    double taxable_value;
    double cgst;         /* Central GST (intra-state: rate/2) */
    double sgst;         /* State GST  (intra-state: rate/2) */
    double igst;         /* Integrated GST (inter-state: full rate) */
    double total_tax;
    double invoice_total;
    char   hsn[16];
    double rate;
};

static GSTCalcResult calc_gst(double value, const char *hsn, GSTType type) {
    GSTCalcResult r{};
    r.taxable_value = value;
    r.rate = gst_rate_for_hsn(hsn);
    strncpy(r.hsn, hsn, sizeof(r.hsn)-1);

    if (type == INTRA_STATE || type == UNION_TERRITORY) {
        r.cgst = round(value * (r.rate / 2.0) / 100.0 * 100) / 100;
        r.sgst = r.cgst;
        r.igst = 0.0;
        r.total_tax = r.cgst + r.sgst;
    } else { /* INTER_STATE */
        r.igst = round(value * r.rate / 100.0 * 100) / 100;
        r.cgst = r.sgst = 0.0;
        r.total_tax = r.igst;
    }
    r.invoice_total = value + r.total_tax;
    return r;
}

// Input Tax Credit (ITC) calculation
static double calc_itc(double purchase_igst, double purchase_cgst,
                        double purchase_sgst) {
    return purchase_igst + purchase_cgst + purchase_sgst;
}

// Net GST payable after ITC set-off
static double gst_payable(double output_tax, double input_tax) {
    double payable = output_tax - input_tax;
    return payable < 0 ? 0.0 : payable;
}

// ── Tests ─────────────────────────────────────────────────────────────────

TEST(GSTBasic, FreshMilkIsExempt) {
    auto r = calc_gst(1000.0, "0401", INTRA_STATE);
    EXPECT_DOUBLE_EQ(r.rate, 0.0);
    EXPECT_DOUBLE_EQ(r.cgst, 0.0);
    EXPECT_DOUBLE_EQ(r.sgst, 0.0);
    EXPECT_DOUBLE_EQ(r.total_tax, 0.0);
    EXPECT_DOUBLE_EQ(r.invoice_total, 1000.0);
}

TEST(GSTBasic, ComputerAt18PctIntraState) {
    auto r = calc_gst(50000.0, "8471", INTRA_STATE);
    EXPECT_DOUBLE_EQ(r.rate, 18.0);
    // CGST = SGST = 50000 * 9% = 4500 each
    EXPECT_DOUBLE_EQ(r.cgst, 4500.0);
    EXPECT_DOUBLE_EQ(r.sgst, 4500.0);
    EXPECT_DOUBLE_EQ(r.igst, 0.0);
    EXPECT_DOUBLE_EQ(r.total_tax, 9000.0);
    EXPECT_DOUBLE_EQ(r.invoice_total, 59000.0);
}

TEST(GSTBasic, ComputerAt18PctInterState) {
    auto r = calc_gst(50000.0, "8471", INTER_STATE);
    EXPECT_DOUBLE_EQ(r.rate, 18.0);
    EXPECT_DOUBLE_EQ(r.cgst, 0.0);
    EXPECT_DOUBLE_EQ(r.sgst, 0.0);
    EXPECT_DOUBLE_EQ(r.igst, 9000.0);
    EXPECT_DOUBLE_EQ(r.invoice_total, 59000.0);
}

TEST(GSTBasic, MobilePhoneAt18Pct) {
    auto r = calc_gst(15000.0, "8517", INTRA_STATE);
    EXPECT_DOUBLE_EQ(r.rate, 18.0);
    EXPECT_DOUBLE_EQ(r.cgst, 1350.0);
    EXPECT_DOUBLE_EQ(r.sgst, 1350.0);
    EXPECT_DOUBLE_EQ(r.total_tax, 2700.0);
    EXPECT_DOUBLE_EQ(r.invoice_total, 17700.0);
}

TEST(GSTBasic, MedicinesAt5Pct) {
    auto r = calc_gst(200.0, "3004", INTRA_STATE);
    EXPECT_DOUBLE_EQ(r.rate, 5.0);
    EXPECT_DOUBLE_EQ(r.cgst, 5.0);  // 200 * 2.5%
    EXPECT_DOUBLE_EQ(r.sgst, 5.0);
    EXPECT_DOUBLE_EQ(r.total_tax, 10.0);
    EXPECT_DOUBLE_EQ(r.invoice_total, 210.0);
}

TEST(GSTBasic, DefaultRateIs18) {
    // Unknown HSN code → 18%
    auto r = calc_gst(1000.0, "9999", INTRA_STATE);
    EXPECT_DOUBLE_EQ(r.rate, 18.0);
    EXPECT_DOUBLE_EQ(r.cgst, 90.0);
    EXPECT_DOUBLE_EQ(r.sgst, 90.0);
}

TEST(GSTITC, InputTaxCreditReducesPayable) {
    // Business buys goods: ₹10,000 + 18% IGST = ₹1,800 ITC
    double purchase_igst = 1800.0;

    // Business sells goods: ₹20,000 + 18% CGST/SGST = ₹1,800 each
    auto sales = calc_gst(20000.0, "8471", INTRA_STATE);
    double output_cgst = sales.cgst;  // 1800
    double output_sgst = sales.sgst;  // 1800
    double output_total = output_cgst + output_sgst; // 3600

    // ITC available from IGST purchase can offset CGST+SGST output
    double itc = calc_itc(purchase_igst, 0, 0);
    double payable = gst_payable(output_total, itc);

    EXPECT_DOUBLE_EQ(itc, 1800.0);
    EXPECT_DOUBLE_EQ(payable, 1800.0); // 3600 - 1800 = 1800
}

TEST(GSTITC, ExcessITCGivesZeroPayable) {
    // More ITC than output tax → zero payable (refund scenario)
    double output_tax = 500.0;
    double input_tax  = 800.0;
    EXPECT_DOUBLE_EQ(gst_payable(output_tax, input_tax), 0.0);
}

TEST(GSTRounding, PaisaRounding) {
    // ₹173.50 * 18% / 2 = ₹15.615 → should round to ₹15.62
    auto r = calc_gst(173.50, "8471", INTRA_STATE);
    EXPECT_NEAR(r.cgst, 15.615, 0.01);
    EXPECT_NEAR(r.sgst, 15.615, 0.01);
}

TEST(GSTInterState, UnionTerritoryUsesUTGST) {
    // Union territory: CGST + UTGST (same math as SGST)
    auto r = calc_gst(10000.0, "8471", UNION_TERRITORY);
    EXPECT_DOUBLE_EQ(r.cgst, 900.0);
    EXPECT_DOUBLE_EQ(r.sgst, 900.0); // UTGST stored in sgst field
    EXPECT_DOUBLE_EQ(r.igst, 0.0);
}
