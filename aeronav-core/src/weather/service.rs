use crate::error::AeroNavError;
use crate::weather::model::{ParsedWeatherReport, WeatherDocument};
use metar_taf_parser::{Language, ParsedReport, format_metar, format_taf, parse};

/// English RustDoc comment.
/// Parses a raw weather report and returns a WeatherDocument containing
/// both the structured report and its natural-language rendering.
pub fn decode_weather_report(
    raw: &str,
    language: Language,
) -> Result<WeatherDocument, AeroNavError> {
    let parsed = parse(raw).map_err(|err| AeroNavError::Parse(err.to_string()))?;

    let (parsed_report, formatted_text) = match parsed {
        ParsedReport::Metar(report) => {
            let formatted = format_metar(&report, language);
            (ParsedWeatherReport::Metar(report), formatted)
        }
        ParsedReport::Taf(report) => {
            let formatted = format_taf(&report, language);
            (ParsedWeatherReport::Taf(Box::from(report)), formatted)
        }
    };

    Ok(WeatherDocument {
        raw_text: raw.to_string(),
        parsed: parsed_report,
        formatted_text,
        language,
    })
}

#[cfg(test)]
mod tests {
    use super::decode_weather_report;
    use metar_taf_parser::Language;

    /// English RustDoc comment.
    /// Verifies that a METAR report can be decoded successfully.
    #[test]
    fn decode_metar_report() {
        let raw = "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG";

        let document = decode_weather_report(raw, Language::En).unwrap();

        assert!(document.is_metar());
        assert_eq!(document.station(), "LIRF");
        assert_eq!(document.report_kind(), "METAR");
        assert!(!document.formatted_text.is_empty());
    }

    /// English RustDoc comment.
    /// Verifies that a TAF report can be decoded successfully.
    #[test]
    fn decode_taf_report() {
        let raw = "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020";

        let document = decode_weather_report(raw, Language::En).unwrap();

        assert!(document.is_taf());
        assert_eq!(document.station(), "LIRF");
        assert_eq!(document.report_kind(), "TAF");
        assert!(!document.formatted_text.is_empty());
    }
}
