#!/bin/bash

set -eE
trap "pkill -P $$" EXIT

start_consumer () {
    trap "pkill mosquitto_sub" EXIT
    SED_INPUT=
    mosquitto_sub -v -t "ping/#" | sed 's/^\([^ ]*\) \([^:]*\):\(.*\)/[consumer\/\1] Received \2:\3/g'
}

start_producer () {
    topic="ping/$(uuid)"
    echo "[producer/$topic] Starting..."
    i=0
    while true
    do
        mosquitto_pub -t $topic -m "ping:$i"
        i=$((i + 1))
        sleep 1
    done
}

main () {
    echo "[main] Hello MQTT"
    start_consumer &
    for i in {1..1..1}
    do
        start_producer &
    done
    sleep 5
    echo "[main] Finishing..."
    exit 0

}

main