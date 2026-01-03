pub mod text;
pub mod json;

use crate::analyzer::Analysis;

pub trait OutputFormatter {
    fn format(&self, analysis: &Analysis) -> String;
}

pub use text::TextFormatter;
pub use json::JsonFormatter;

