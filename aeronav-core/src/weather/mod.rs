pub mod model;
pub mod service;

pub use model::{ParsedWeatherReport, WeatherDocument};
pub use service::decode_weather_report;
