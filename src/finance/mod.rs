// SigmaOS Finance Module
// India Stack financial calculations (GST, TDS, Income Tax)

pub mod gst;
pub mod tds;

pub use gst::{GoodsType, GstCalculator, GstRate, GstRegime, GstResult, GstState};
pub use tds::{TdsCalculator, TdsResult, TdsSection};
