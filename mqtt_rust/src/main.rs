use std::sync::Arc;
use uuid::Uuid;

use log::info;

use paho_mqtt::async_client::AsyncClient as MQTTClient;
use paho_mqtt::create_options::CreateOptions;
use paho_mqtt::Message;

async fn start_consumer(mut client: MQTTClient) {
    info!("[consumer] Starting...");
    client
        .subscribe("ping/#", 1)
        .await
        .expect("[consumer] Unable to subscribe to ping/#");
    let stream = client.get_stream(1000);
    while let Ok(Some(message)) = stream.recv().await {
        info!(
            "[consumer/{}] Received {}",
            message.topic(),
            message.payload_str()
        );
    }
}

async fn start_producer(client: Arc<MQTTClient>) {
    let topic_id = Uuid::new_v4();
    let topic = String::from(format!("ping/{}", topic_id));
    info!("[producer/{topic}] Starting...");
    let mut nonce: i32 = 0;
    loop {
        let payload = format!("ping:{nonce}");
        // info!("[producer/{}] Sending {}", topic, payload);
        client.publish(Message::new(topic.clone(), payload, 1))
            .await
            .ok();
        nonce += 1;
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_env().unwrap();
    info!("[main] Hello MQTT");
    let options = CreateOptions::from("tcp://127.0.0.1");
    let client = MQTTClient::new(options).expect("[main] MQTT could not create client");
    client
        .connect(None)
        .await
        .expect("[main] MQTT Client could not connect");


    tokio::spawn(start_consumer(client.clone()));
    let client_prod = Arc::new(client);

    for _ in 0..10 {
        tokio::spawn(start_producer(Arc::clone(&client_prod)));
    }

    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    client_prod.disconnect(None)
        .await
        .expect("[main] MQTT disconnect failed");
    // client_cons.disconnect(None).expect("");
}
