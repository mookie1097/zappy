#!/bin/bash

ipaddr="$(ifconfig | grep -Eo 'inet (addr:)?([0-9]*\.){3}[0-9]*' | grep -Eo '([0-9]*\.){3}[0-9]*' | grep -v '127.0.0.1')"
#echo $ipaddr
while [[ $ipaddr == "" ]] ; do
    sleep 3
    ipaddr="$(ifconfig | grep -Eo 'inet (addr:)?([0-9]*\.){3}[0-9]*' | grep -Eo '([0-9]*\.){3}[0-9]*' | grep -v '127.0.0.1')"
    #echo $ipaddr
done

hciconfig hci0 up
hciconfig hci0 leadv 3
cmd=$(python3 beacon.py $ipaddr:8000)
#echo $cmd
exec $cmd