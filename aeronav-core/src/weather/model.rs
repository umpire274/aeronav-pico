use metar_taf_parser::Language;
use metar_taf_parser::metar::models::Metar;
use metar_taf_parser::taf::models::Taf;

/// English RustDoc comment.
/// Represents a parsed weather report handled by the application.
#[derive(Debug)]
pub enum ParsedWeatherReport {
    Metar(Box<Metar>),
    Taf(Box<Taf>),
}

/// English RustDoc comment.
/// Represents a weather report together with its natural-language rendering.
#[derive(Debug)]
pub struct WeatherDocument {
    pub raw_text: String,
    pub parsed: ParsedWeatherReport,
    pub formatted_text: String,
    pub language: Language,
}

impl WeatherDocument {
    /// English RustDoc comment.
    /// Returns true when the document contains a METAR report.
    pub fn is_metar(&self) -> bool {
        matches!(self.parsed, ParsedWeatherReport::Metar(_))
    }

    /// English RustDoc comment.
    /// Returns true when the document contains a TAF report.
    pub fn is_taf(&self) -> bool {
        matches!(self.parsed, ParsedWeatherReport::Taf(_))
    }

    /// English RustDoc comment.
    /// Returns the station identifier of the parsed weather report.
    pub fn station(&self) -> &str {
        match &self.parsed {
            ParsedWeatherReport::Metar(report) => &report.station,
            ParsedWeatherReport::Taf(report) => &report.station,
        }
    }

    /// English RustDoc comment.
    /// Returns the report kind as a static string.
    pub fn report_kind(&self) -> &'static str {
        match &self.parsed {
            ParsedWeatherReport::Metar(_) => "METAR",
            ParsedWeatherReport::Taf(_) => "TAF",
        }
    }
}
