use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Opts {
    #[envconfig(from = "WEATHER_MEASUREMENT_INTERVAL_IN_SECONDS", default = "120")]
    pub measurement_interval_in_seconds: u16,

    #[envconfig(from = "WEATHER_OPENWEATHERMAP_LONGITUDE")]
    pub open_weather_longitude: String,

    #[envconfig(from = "WEATHER_OPENWEATHERMAP_LATITUDE")]
    pub open_weather_latitude: String,

    #[envconfig(from = "WEATHER_OPENWEATHERMAP_API_KEY")]
    pub open_weather_api_key: String,

    #[envconfig(from = "WEATHER_MQTT_BROKER_HOST")]
    pub mqtt_borker_host: String,

    #[envconfig(from = "WEATHER_MQTT_BROKER_PORT")]
    pub mqtt_borker_port: u16,

    #[envconfig(from = "WEATHER_MQTT_TOPIC")]
    pub mqtt_topic: String,
}
