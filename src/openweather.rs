use serde::Deserialize;

use crate::weather_message::WeatherMessage;

pub struct OpenWeather {
    api_key: String,
    latitude: String,
    longitude: String,
}

impl OpenWeather {
    pub fn new(api_key: String, latitude: String, longitude: String) -> Self {
        Self {
            api_key,
            latitude,
            longitude,
        }
    }

    pub async fn fetch_weather_data(&self) -> Result<WeatherMessage, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = format!(
            "{}?lat={}&lon={}&exclude=minutely,hourly,daily,alerts&appid={}&units=metric",
            BASE_URL, self.latitude, self.longitude, self.api_key
        );
        let response = client
            .get(url)
            .send()
            .await?
            .json::<OpenWeatherResponse>()
            .await;

        match response {
            Ok(response) => Ok(WeatherMessage {
                temperatur: response.current.temp,
                humidity: response.current.humidity,
                pressure: response.current.pressure,
                clouds: response.current.clouds,
                weather: response.current.weather[0].main.clone(),
            }),
            Err(e) => return Err(Box::new(e)),
        }
    }
}

const BASE_URL: &str = "https://api.openweathermap.org/data/3.0/onecall";

#[derive(Deserialize)]
struct OpenWeatherResponse {
    current: OpenWeatherCurrent,
}

#[derive(Deserialize)]
struct OpenWeatherCurrent {
    temp: f32,
    pressure: i16,
    humidity: i16,
    clouds: i16,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct Weather {
    main: String,
}
