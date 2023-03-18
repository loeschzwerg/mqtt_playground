use paho_mqtt::client::Client as MQTTClient;
use paho_mqtt::create_options::CreateOptions;
use paho_mqtt::Message;
use std::sync::Arc;
use uuid::Uuid;

async fn start_consumer(client: Arc<MQTTClient>) {
    println!("[consumer] Starting...");
    client
        .subscribe("ping/#", 1)
        .expect("[consumer] Unable to subscribe to ping/#");
    while let Ok(Some(message)) = client.start_consuming().recv() {
        println!(
            "[consumer/{}] Received {}",
            message.topic(),
            message.payload_str()
        );
    }
}

async fn start_producer(client: Arc<MQTTClient>) {
    let topic_id = Uuid::new_v4();
    let topic = String::from(format!("ping/{}", topic_id));
    println!("[producer/{topic}] Starting...");
    let mut nonce: i32 = 0;
    loop {
        let payload = format!("ping:{nonce}");
        println!("[producer/{}] Sending {}", topic, payload);
        client.publish(Message::new(topic.clone(), payload, 1)).ok();
        nonce += 1;
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }
}

#[tokio::main]
async fn main() {
    println!("[main] Hello MQTT");
    let options = CreateOptions::from("tcp://127.0.0.1");
    let client = MQTTClient::new(options).expect("[main] MQTT could not create client");
    client
        .connect(None)
        .expect("[main] MQTT Client could not connect");

    let client_prod = Arc::new(client.clone());
    let client_cons = Arc::new(client);

    tokio::spawn(start_consumer(Arc::clone(&client_cons)));
    for _ in 0..10 {
        tokio::spawn(start_producer(Arc::clone(&client_prod)));
    }

    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    client_prod.disconnect(None).expect("[main] MQTT disconnect failed");
    // client_cons.disconnect(None).expect("[main] MQTT disconnect failed");
}
