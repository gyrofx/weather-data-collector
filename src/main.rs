mod mqtt;
mod openweather;
mod opts;
mod weather_message;
use log::{error, info};
use std::{panic, process, time::Duration};

use async_std::task;
use envconfig::Envconfig;
use opts::Opts;

use mqtt::Mqtt;

use crate::openweather::OpenWeather;

#[async_std::main]
async fn main() {
    env_logger::init();

    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        process::exit(1);
    }));

    let config = Opts::init_from_env().unwrap();

    info!("Starting OpenWeather Data Collector");

    let open_weather: OpenWeather = OpenWeather::new(
        config.open_weather_api_key,
        config.open_weather_latitude,
        config.open_weather_longitude,
    );

    let (mqtt, mut eventloop) = Mqtt::new(
        &config.mqtt_borker_host,
        config.mqtt_borker_port,
        &config.mqtt_topic,
    );

    task::spawn(async move {
        loop {
            let message = open_weather.fetch_weather_data().await.unwrap();

            match mqtt.publish_message(&message).await {
                Ok(_) => info!("Published to {:?}", message),
                Err(e) => error!("Error: failed to publish message: {}", e),
            }

            task::sleep(Duration::from_secs(
                config.measurement_interval_in_seconds.into(),
            ))
            .await;
        }
    });

    loop {
        break match eventloop.poll().await {
            Ok(_) => {
                continue;
            }
            Err(e) => {
                error!("Error: failed to poll eventloop: {}", e)
            }
        };
    }
}
