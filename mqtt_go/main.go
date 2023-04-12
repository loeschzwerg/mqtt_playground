package main

import (
	"fmt"
	"os"
	"time"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	"github.com/google/uuid"
	"golang.org/x/exp/slog" // built-in log does not have a default implementation for log-levels...
)

var log = slog.New(slog.NewTextHandler(os.Stdout))

func acquire_connection(broker string, port int) mqtt.Client {
	opts := mqtt.NewClientOptions()
	opts.AddBroker(fmt.Sprintf("tcp://%s:%d", broker, port))
	opts.SetClientID("go_mqtt_client")
	client := mqtt.NewClient(opts)

	if token := client.Connect(); token.Wait() && token.Error() != nil {
		log.Error("[main] Failed to connect to MQTT broker")
		panic(token.Error())
	}

	return client
}

func start_consumer(client mqtt.Client) {
	log.Info("[consumer] Subscribing...")
	client.Subscribe("ping/#", 1, func(c mqtt.Client, m mqtt.Message) {
		log.Info(
			fmt.Sprintf(
				"[consumer/%v] %s",
				m.Topic(),
				m.Payload()))
	})
}

func start_producer(client mqtt.Client) {
	var uuid string = uuid.NewString()
	var topic string = fmt.Sprintf("ping/%v", uuid)
	log.Info(fmt.Sprintf("[producer %s] Producing...", topic))
	var nonce int = 0
	for {
		client.Publish(topic, 1, false, fmt.Sprintf("ping:%d", nonce))
		nonce += 1
		time.Sleep(time.Second)
	}
}

func main() {
	log.Info("[main] Hello MQTT")
	client := acquire_connection("127.0.0.1", 1883)
	go start_consumer(client)
	for threads := 0; threads < 10; threads++ {
		go start_producer(client)
	}
	time.Sleep(10 * time.Second)
	log.Info("[main] Finishing...")
	client.Disconnect(10)
}
