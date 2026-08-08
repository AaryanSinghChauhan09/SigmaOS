// SigmaOS Finance Module
// India Stack financial calculations (GST, TDS, Income Tax)

pub mod gst;
pub mod professions;
pub mod tds;

pub use gst::{GoodsType, GstCalculator, GstRate, GstRegime, GstResult, GstState};
pub use professions::{
    AssetClass, IndianCrop, KanoonCalculator, KrishiCalculator, LimitationType, VyapaarCalculator,
    ChikitshakCalculator, AbhiyantaCalculator, ConcreteGrade,
};
pub use tds::{TdsCalculator, TdsResult, TdsSection};
