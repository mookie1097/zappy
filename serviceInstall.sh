#!/bin/bash
#python setup stuff??? idk dont run this itll probably break shit
pushd zappyservice

sudo apt update
sudo apt install python3-venv
pushd python ## in python dir now ##
    echo "init venv"
    python3 -m venv env

    source env/bin/activate
        echo "install into venv"
        pip3 install --upgrade pip3
        pip3 install websockets pillow 
        pip3 install adafruit-circuitpython-lis3dh adafruit-circuitpython-ssd1306
    deactivate
popd

echo "install service"
bash setup
