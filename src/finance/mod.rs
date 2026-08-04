#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Finance Module
// India Stack financial calculations (GST, TDS, Income Tax) and Professional Utilities

pub mod gst;
pub mod professions;
pub mod tds;

pub use gst::{GoodsType, GstCalculator, GstRate, GstRegime, GstResult, GstState};
pub use professions::{
    EngineeringCalculators, FinancialProfessionCalculators, MedicalDoctorCalculators,
    DepreciationMethod, TaxRegime,
};
pub use tds::{TdsCalculator, TdsResult, TdsSection};
