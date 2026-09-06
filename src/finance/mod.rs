#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Finance Module
// India Stack financial calculations (GST, TDS, Income Tax)

pub mod data_commerce;
pub mod gst;
pub mod professions;
pub mod tds;

pub use data_commerce::{
    CommercialPricingModel, DataClassificationTag, DataCommerceDlpEngine,
    DataCommerceTelemetryMeter, DataProductListing, EntitlementCertificate,
    FedoraDataMarketplaceEngine, RhsmEntitlementEngine, SlaLevel, UsageRecord,
};
pub use gst::{GoodsType, GstCalculator, GstRate, GstRegime, GstResult, GstState};
pub use professions::{
    AssetClass, IndianCrop, KanoonCalculator, KrishiCalculator, LimitationType, VyapaarCalculator,
    ChikitshakCalculator, AbhiyantaCalculator, ConcreteGrade,
};
pub use tds::{TdsCalculator, TdsResult, TdsSection};
