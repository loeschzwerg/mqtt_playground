# MQTT Playground

> playground to evaluate common coding paradigms in different languages

### Task
Write a program that **connects to a local MQTT instance**.
Once connection is established, **start a consumer thread**, that listens on the `"ping/#"` topic.
Continuously listen on the the topic.

Further, include a **logging facade implementation**. On program start, log a message

> [main] Hello MQTT

Everytime a message is received on any of the ping topics, log a message

> [consumer/ping/{topic}] Received {message}

Next, the main thread should **spawn 10 producer threads**.
Every producer creates its own topic to publish on: `ping/<uuid>`.
Every second, a producer sends a message `ping:<nonce>` starting with `ping:0`.
The `nonce` is then incremented, and the Thread is put to sleep for 1 second.
The thread should then loop back to the publish.

After all 11 threads are spawned, the main thread sleeps for 10 seconds, closes the connection, and stops the program.

### Evaluation of practices
The following coding practices may be evaluated:
- use a library (MQTT), which is similar in implementation on most platforms
- use a logging facade, implement a simple logger, and configure the log-level
- use a common UUID implementation
- use the StringBuilder pattern
- spawn a thread
- handle continuous streams without disruption
- write into a log output
- spawn n threads, looping indefinitely
- put spawned threads to sleep
- asynchronously use a single connection (depends on the MQTT implementation)


### Prerequisites

Use an MQTT Broker. 
An example would be `mosquitto`.
```bash
# start as a daemon
systemctl start mosquitto

# start in a shell
mosquitto

# continuously log all messages published to the topic "ping/#"
mosquitto_sub -t "ping/#"

# log all topics and messages
mosquitto_sub -v -t "#"

# publish a message on "ping/1"
mosquitto_pub -t "ping/1" -m "ping:0"

```