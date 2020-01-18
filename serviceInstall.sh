#!/bin/bash
#python setup stuff??? idk dont run this itll probably break shit

sudo apt update
sudo apt install python3-venv

echo "init venv"
python3 -m venv env

source env/bin/activate

echo "install into venv"

pip3 install --upgrade pip 

pip install websockets # pylint # buttplug nodeenv autopep8 flask sh arrow

pip3 install adafruit-circuitpython-lis3dh adafruit-circuitpython-ssd1306


deactivate


echo "install service"
bash service/setup
