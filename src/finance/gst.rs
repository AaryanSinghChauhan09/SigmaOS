#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// India Stack - GST Engine
// Goods and Services Tax calculation for Indian regulatory compliance

// (no_std only applicable at crate root - removed)

use std::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GstRate {
    Rate0,
    Rate5,
    Rate12,
    Rate18,
    Rate28,
    Rate28WithCess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GstState {
    AndhraPradesh,
    Karnataka,
    Maharashtra,
    Gujarat,
    TamilNadu,
    UttarPradesh,
    WestBengal,
    Delhi,
    Rajasthan,
    Kerala,
    Punjab,
    Haryana,
    MadhyaPradesh,
    Bihar,
    Odisha,
    Assam,
    Telangana,
    Jharkhand,
    Uttarakhand,
    HimachalPradesh,
    Chhattisgarh,
    Goa,
    JammuKashmir,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GstRegime {
    IntraState {
        state: GstState,
    },
    InterState {
        from_state: GstState,
        to_state: GstState,
    },
    Export {
        destination_country: &'static str,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct GstResult {
    pub cgst_paise: u64,
    pub sgst_paise: u64,
    pub igst_paise: u64,
    pub cess_paise: u64,
    pub total_paise: u64,
    pub lut_required: bool,
}

pub struct GstCalculator;

impl GstCalculator {
    pub fn calculate_gst(base_amount_paise: u64, rate: GstRate, regime: GstRegime) -> GstResult {
        let (rate_percent, cess_percent) = match rate {
            GstRate::Rate0 => (0.0, 0.0),
            GstRate::Rate5 => (5.0, 0.0),
            GstRate::Rate12 => (12.0, 0.0),
            GstRate::Rate18 => (18.0, 0.0),
            GstRate::Rate28 => (28.0, 0.0),
            GstRate::Rate28WithCess => (28.0, 15.0), // 15% cess on 28%
        };

        let gst_amount = (base_amount_paise as f64 * rate_percent / 100.0) as u64;
        let cess_amount = if cess_percent > 0.0 {
            (base_amount_paise as f64 * cess_percent / 100.0) as u64
        } else {
            0
        };

        match regime {
            GstRegime::IntraState { .. } => {
                let half_gst = gst_amount / 2;
                GstResult {
                    cgst_paise: half_gst,
                    sgst_paise: half_gst,
                    igst_paise: 0,
                    cess_paise: cess_amount,
                    total_paise: base_amount_paise + gst_amount + cess_amount,
                    lut_required: false,
                }
            }
            GstRegime::InterState { .. } => GstResult {
                cgst_paise: 0,
                sgst_paise: 0,
                igst_paise: gst_amount,
                cess_paise: cess_amount,
                total_paise: base_amount_paise + gst_amount + cess_amount,
                lut_required: false,
            },
            GstRegime::Export { .. } => GstResult {
                cgst_paise: 0,
                sgst_paise: 0,
                igst_paise: 0,
                cess_paise: 0,
                total_paise: base_amount_paise,
                lut_required: true,
            },
        }
    }

    pub fn get_rate_for_goods(goods_type: GoodsType) -> GstRate {
        match goods_type {
            GoodsType::EssentialFood => GstRate::Rate0,
            GoodsType::BasicCommodities => GstRate::Rate5,
            GoodsType::ProcessedFood => GstRate::Rate12,
            GoodsType::StandardGoods => GstRate::Rate18,
            GoodsType::LuxuryGoods => GstRate::Rate28,
            GoodsType::LuxuryWithCess => GstRate::Rate28WithCess,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoodsType {
    EssentialFood,
    BasicCommodities,
    ProcessedFood,
    StandardGoods,
    LuxuryGoods,
    LuxuryWithCess,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intra_state_gst() {
        let result = GstCalculator::calculate_gst(
            100_000,
            GstRate::Rate18,
            GstRegime::IntraState {
                state: GstState::Maharashtra,
            },
        );

        assert_eq!(result.cgst_paise, 9_000);
        assert_eq!(result.sgst_paise, 9_000);
        assert_eq!(result.igst_paise, 0);
        assert_eq!(result.total_paise, 118_000);
    }

    #[test]
    fn test_inter_state_gst() {
        let result = GstCalculator::calculate_gst(
            100_000,
            GstRate::Rate12,
            GstRegime::InterState {
                from_state: GstState::Karnataka,
                to_state: GstState::Gujarat,
            },
        );

        assert_eq!(result.cgst_paise, 0);
        assert_eq!(result.sgst_paise, 0);
        assert_eq!(result.igst_paise, 12_000);
        assert_eq!(result.total_paise, 112_000);
    }

    #[test]
    fn test_export_gst() {
        let result = GstCalculator::calculate_gst(
            500_000,
            GstRate::Rate18,
            GstRegime::Export {
                destination_country: "USA",
            },
        );

        assert_eq!(result.cgst_paise, 0);
        assert_eq!(result.sgst_paise, 0);
        assert_eq!(result.igst_paise, 0);
        assert_eq!(result.total_paise, 500_000);
        assert!(result.lut_required);
    }

    #[test]
    fn test_gst_with_cess() {
        let result = GstCalculator::calculate_gst(
            100_000,
            GstRate::Rate28WithCess,
            GstRegime::IntraState {
                state: GstState::Maharashtra,
            },
        );

        assert!(result.cess_paise > 0);
        assert_eq!(result.total_paise, 100_000 + 28_000 + 15_000);
    }

    #[test]
    fn test_goods_type_mapping() {
        assert_eq!(
            GstCalculator::get_rate_for_goods(GoodsType::EssentialFood),
            GstRate::Rate0
        );
        assert_eq!(
            GstCalculator::get_rate_for_goods(GoodsType::BasicCommodities),
            GstRate::Rate5
        );
        assert_eq!(
            GstCalculator::get_rate_for_goods(GoodsType::ProcessedFood),
            GstRate::Rate12
        );
        assert_eq!(
            GstCalculator::get_rate_for_goods(GoodsType::StandardGoods),
            GstRate::Rate18
        );
        assert_eq!(
            GstCalculator::get_rate_for_goods(GoodsType::LuxuryGoods),
            GstRate::Rate28
        );
    }
}
