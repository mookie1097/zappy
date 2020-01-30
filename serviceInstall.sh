#!/bin/bash
#python setup stuff??? idk dont run this itll probably break shit
pushd zappyservice

sudo apt update
yes | sudo apt install python3-venv libopenjp2-7-dev libtiff5 apache2
pushd python ## in python dir now ##
    echo "init venv"
    python3 -m venv env

    source env/bin/activate
        echo "install into venv"
        pip3 install --upgrade pip3
        echo "installing websockets and pillow"
        pip3 install websockets pillow 
        echo "installing adafruit libs"
        pip3 install adafruit-circuitpython-lis3dh adafruit-circuitpython-ssd1306
    deactivate
popd

sudo cp index.html index.js /var/www/html/ 

echo "install service"
bash setup
popd
