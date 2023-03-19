#!/usr/bin/python3

from sys import exit
from time import sleep
from threading import Thread, currentThread
from uuid import uuid4
import logging as log

import paho.mqtt.client as mqttc
from paho.mqtt.client import Client

log.basicConfig(level=log.INFO)

client = Client(client_id="mqtt_python", protocol=mqttc.MQTTv5)

@client.connect_fail_callback()
def connection_failed(_client, _user_data):
    log.error("[main] connection failed")

@client.connect_callback()
def connection_established(_client, _userdata, _flags, reasoncode, properties):
    log.info(f"[main] connection established {reasoncode}")

@client.message_callback()
def log_message(_client, _userdata, message):
    log.info("[consumer/{}] Received {}".format(message.topic, str(message.payload, 'UTF-8')))

@client.disconnect_callback()
def disconnect(client, userdata, reasoncode, properties):
    log.info(f"[main] Disconnected {reasoncode}")

# @client.publish_callback()
# def publish(client, userdata, mid):
#    print("Published", mid)


def start_consumer():
    log.info("[consumer] Subscribing...")
    # client.message_callback_add("ping/#", log_message)
    result, _message_id = client.subscribe("#")
    assert(result is mqttc.MQTT_ERR_SUCCESS)

def start_producer():
    topic = "ping/" + str(uuid4())
    log.info(f"[producer/{topic}] Starting...")
    nonce = 0
    while client.is_connected(): # Threads will not finish otherwise
        payload = f"ping:{nonce}"
        log.debug(f"[producer/{topic}] Publishing {payload}")
        client.publish(topic, payload)
        nonce += 1
        sleep(1)


def main():
    log.info("[main] Hello MQTT")
    client.connect(host="localhost") # connect_async doesn't output correctly
    client.loop_start()
    start_consumer()
    for _ in range(10):
        t = Thread(target=start_producer)
        t.start()

    sleep(15)
    client.loop_stop() # apparently needed for graceful disconnect
    client.disconnect()
    exit(0)

if __name__ == "__main__":
    main()