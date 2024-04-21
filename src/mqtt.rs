use std::time::Duration;

use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde::Serialize;

pub struct Mqtt {
    client: AsyncClient,
    topic: String,
}

impl Mqtt {
    pub fn new(host: &String, port: u16, topic: &String) -> (Mqtt, rumqttc::EventLoop) {
        let mut mqttoptions = MqttOptions::new("weather-data-collector", host, port);
        mqttoptions.set_keep_alive(Duration::from_secs(30));

        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

        (
            Mqtt {
                client,
                topic: topic.to_string(),
            },
            eventloop,
        )
    }

    pub async fn publish_message<T: Serialize>(
        &self,
        payload: T,
    ) -> Result<(), rumqttc::ClientError> {
        let payload_as_bytes = serde_json::to_string(&payload)
            .expect("Serialization failed")
            .into_bytes();
        self.client
            .publish(&self.topic, QoS::ExactlyOnce, false, payload_as_bytes)
            .await
    }
}
