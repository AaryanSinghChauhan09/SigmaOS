# 🇮🇳 SigmaOS India Stack

> SigmaOS is the **world's first sovereign OS with native Indian regulatory compliance built into the kernel**. GST calculation, TDS deduction, Income Tax computation, UPI payment flows, and 22-language support are not plugins — they are OS primitives.

---

## 🏛️ India Stack Overview

```
┌──────────────────────────────────────────────────────────┐
│                   India Stack Layer                       │
├──────────────┬───────────────┬──────────────────────────┤
│  Finance     │  Identity     │  Language                 │
│  Module      │  Module       │  Module                   │
│              │               │                           │
│ • GST Engine │ • Aadhaar API │ • 22 Scheduled Languages  │
│ • TDS Engine │ • DigiLocker  │ • Indic Font Rendering    │
│ • Income Tax │ • DIGI Yatra  │ • Voice Input (on-device) │
│ • UPI Gen    │ • PAN Verify  │ • RTL Support (Urdu)      │
└──────────────┴───────────────┴──────────────────────────┘
                        │
               S-SEC Capability Gate
               (All India Stack APIs are capability-gated)
```

---

## 💰 Finance Module (sigma_finance)

### GST Engine

India's Goods and Services Tax system with all three tax regimes:

#### Intra-State (CGST + SGST)
```rust
let gst = GstCalculator::calculate_gst(
    100_000,  // base_amount_paise (₹1000.00)
    GstRate::Rate18,  // 18% GST
    GstRegime::IntraState { state: GstState::Maharashtra }
);
// → GstResult { cgst_paise: 9_000, sgst_paise: 9_000, total_paise: 118_000 }
```

#### Inter-State (IGST)
```rust
let gst = GstCalculator::calculate_gst(
    100_000,
    GstRate::Rate12,
    GstRegime::InterState {
        from_state: GstState::Karnataka,
        to_state: GstState::Gujarat
    }
);
// → GstResult { igst_paise: 12_000, total_paise: 112_000 }
```

#### Export (Zero-Rated Supply)
```rust
let gst = GstCalculator::calculate_gst(
    500_000,
    GstRate::Rate18,
    GstRegime::Export { destination_country: "USA" }
);
// → GstResult { gst_paise: 0, total_paise: 500_000, lut_required: true }
```

### GST Rates Supported

| Rate | Applicable Goods | Example |
|------|-----------------|---------|
| 0% | Essential food, healthcare | Rice, dal, medicines |
| 5% | Basic commodities | Edible oils, tea |
| 12% | Processed food, business goods | Frozen food, computers |
| 18% | Standard goods & services | Electronics, restaurants |
| 28% | Luxury & sin goods | Cars, tobacco, luxury hotels |
| 28% + Cess | Luxury vehicles, tobacco | Cigarettes, SUVs |

---

### TDS Engine

Tax Deducted at Source covering all major sections:

```rust
let tds = TdsCalculator::compute_tds(
    TdsSection::Section194C { is_contractor: true },
    payment_amount_paise: 5_000_000,  // ₹50,000
    pan_available: true,
);
// → TdsResult { tds_paise: 100_000, rate: 0.02, threshold_crossed: true }
```

#### TDS Sections Implemented

| Section | Nature of Payment | Rate (with PAN) | Rate (without PAN) |
|---------|------------------|-----------------|--------------------|
| 192 | Salary | As per slab | — |
| 194A | Interest | 10% | 20% |
| 194B | Lottery winnings | 30% | 30% |
| 194C | Contractor payments | 1%/2% | 20% |
| 194D | Insurance commission | 5% | 20% |
| 194H | Commission / brokerage | 5% | 20% |
| 194I | Rent | 10% | 20% |
| 194J | Professional fees | 10% | 20% |
| 194Q | Purchase of goods | 0.1% | 5% |
| 195 | Non-resident payments | Per treaty | 20% |

---

### Income Tax Calculator

Full FY 2024-25 computation for both regimes:

#### New Tax Regime (Default from FY 2023-24)
```rust
let tax = IncomeTaxCalculator::calculate_tax(
    TaxpayerProfile {
        gross_income_paise: 80_00_00_00,  // ₹80 lakh
        regime: TaxRegime::NewRegime2024,
        age_category: AgeCategory::BelowSixty,
        deductions: Deductions::none(),
    }
);
```

**New Regime Slabs (FY 2024-25):**

| Income Slab | Tax Rate |
|-------------|----------|
| Up to ₹3 lakh | Nil |
| ₹3L – ₹6L | 5% |
| ₹6L – ₹9L | 10% |
| ₹9L – ₹12L | 15% |
| ₹12L – ₹15L | 20% |
| Above ₹15L | 30% |

#### Old Tax Regime
**Deductions supported:** 80C (₹1.5L), 80D (health insurance), 80E (education loan), 80G (donations), HRA, LTA, Standard deduction ₹50,000.

#### Section 87A Rebate
- New regime: Rebate up to ₹25,000 for income ≤ ₹7 lakh
- Old regime: Rebate up to ₹12,500 for income ≤ ₹5 lakh

#### Surcharge Slabs

| Income | Surcharge Rate |
|--------|---------------|
| ₹50L – ₹1Cr | 10% |
| ₹1Cr – ₹2Cr | 15% |
| ₹2Cr – ₹5Cr | 25% |
| Above ₹5Cr | 37% (old) / 25% (new) |

---

### UPI Generator

NPCI-compliant UPI deep-link generation:

```rust
let upi_request = UpiRequestGenerator::generate_payment_request(
    UpiPaymentRequest {
        payee_vpa: "merchant@upi",
        payee_name: "Raj Traders",
        amount_paise: 50_000,  // ₹500.00
        currency: "INR",
        transaction_note: "Payment for invoice #1234",
        transaction_ref: "INV1234",
    }
);
// → "upi://pay?pa=merchant@upi&pn=Raj%20Traders&am=500.00&cu=INR&tn=Payment%20for%20invoice%20%231234&tr=INV1234"
```

**UPI Apps Supported:** PhonePe, Google Pay, Paytm, BHIM, Amazon Pay (deep-link compatible with all).

---

### Number Formatting

Indian numbering system (Lakh/Crore):

```rust
InNumberFormat::format_inr(1_00_00_000)  // "₹1,00,00,000"
InNumberFormat::format_words(75_50_000)  // "Seventy-Five Lakh Fifty Thousand"
InNumberFormat::format_short(1_00_00_000)  // "₹1Cr"
InNumberFormat::format_short(5_50_000)     // "₹5.5L"
```

---

## 🗣️ Language Module

### Supported Languages

All 22 languages listed in the Eighth Schedule of the Indian Constitution:

| # | Language | Script | RTL | Status |
|---|----------|--------|-----|--------|
| 1 | Hindi | Devanagari | ❌ | ✅ |
| 2 | Bengali | Bengali | ❌ | ✅ |
| 3 | Marathi | Devanagari | ❌ | ✅ |
| 4 | Telugu | Telugu | ❌ | ✅ |
| 5 | Tamil | Tamil | ❌ | ✅ |
| 6 | Gujarati | Gujarati | ❌ | ✅ |
| 7 | Urdu | Nastaliq | ✅ | ✅ |
| 8 | Kannada | Kannada | ❌ | ✅ |
| 9 | Odia | Odia | ❌ | ✅ |
| 10 | Malayalam | Malayalam | ❌ | ✅ |
| 11 | Punjabi | Gurmukhi | ❌ | ✅ |
| 12 | Assamese | Bengali | ❌ | ✅ |
| 13 | Maithili | Devanagari | ❌ | 🔄 |
| 14 | Sanskrit | Devanagari | ❌ | 🔄 |
| 15 | Sindhi | Devanagari/Arabic | Both | ⬜ |
| 16 | Nepali | Devanagari | ❌ | 🔄 |
| 17 | Konkani | Devanagari | ❌ | ⬜ |
| 18 | Manipuri | Meitei Mayek | ❌ | ⬜ |
| 19 | Bodo | Devanagari | ❌ | ⬜ |
| 20 | Dogri | Devanagari | ❌ | ⬜ |
| 21 | Kashmiri | Sharada/Devanagari | Both | ⬜ |
| 22 | Santhali | Ol Chiki | ❌ | ⬜ |

### Font Rendering

- **HarfBuzz-compatible** shaping engine for complex Indic scripts
- **OpenType** features: conjuncts, matras, half-forms, anusvara
- **Noto Fonts** embedded: NotoSansDevanagari, NotoSansTamil, NotoSansTelugu, etc.
- **Variable font** support for responsive UI scaling

---

## 🪪 Identity Module

### Aadhaar Integration

Privacy-preserving Aadhaar verification using **Virtual IDs (VID)**:

```rust
let verifier = AadhaarVerifier::new(uidai_public_key);
let result = verifier.verify_vid(&vid, &otp)?;
// → Only returns boolean; no raw Aadhaar number stored anywhere
```

### DigiLocker Integration

Sovereign document access compliant with MeitY specifications:
- Document fetch via DigiLocker API
- Documents stored in SigmaVault (PQC-encrypted)
- Offline access: documents cached locally with Dilithium-5 signature

---

## 🔗 Related Pages

- [Security Framework](Security_Framework) — Capability-gated India Stack APIs
- [Sigma AI Agents](Sigma_AI_Agents) — On-device Indic language AI
- [Maturity & Distro-Parity Roadmap](Maturity_Parity_Roadmap) — Phase H: India Stack
- [Advanced Absorption Matrix](Advanced_Absorption) — India-specific app absorption


---
## Merged from India-Stack.md
# India Stack Integration

SigmaOS integrates with India Stack APIs for sovereign digital infrastructure.

## Components

1. **ABDM** (Ayushman Bharat Digital Mission): Health records

2. **UPI**: Payment deeplink handling

3. **GST**: Billing and GSTR filing automation

4. **DigiLocker**: Document verification API client

5. **NavIC**: Satellite navigation integration for GPS-denied areas
