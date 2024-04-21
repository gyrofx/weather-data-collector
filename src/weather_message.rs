use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WeatherMessage {
    pub temperatur: f32,
    pub humidity: i16,
    pub pressure: i16,
    pub clouds: i16,
    pub weather: String,
}
