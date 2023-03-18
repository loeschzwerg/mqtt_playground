#!/bin/bash

echo "... Starting containerized mosquitto"
# https://hub.docker.com/_/eclipse-mosquitto/
# if [ -f mosquitto.conf ] ; then
#     docker run -it -p 1883:1883 -p 9001:9001 -v mosquitto.conf:/mosquitto/config/mosquitto.conf -v /mosquitto/log eclipse-mosquitto
# else
#     docker run -it -p 1883:1883 -p 9001:9001 -v /mosquitto/log eclipse-mosquitto
# fi

# systemctl start mosquitto

mosquitto_sub -t "#" &
mosquitto_pub -t "hello" -m "Server running. Exiting start script.."

echo Exit
