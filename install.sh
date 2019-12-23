#!/bin/bash

sudo apt update
sudo apt install python3-venv curl g++

echo "init venv"
python3 -m venv env

source env/bin/activate

pip install --upgrade pip 

pip install websockets pylint buttplug # nodeenv autopep8 flask sh arrow

deactivate
source env/bin/activate

#install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh





#not going to use python node unless necessicary 
#nodeenv --python-virtualenv --prebuilt
#pip install nodeenv
#nodeenv --python-virtualenv
#deactivate
#source env/bin/activate  # Reactivate
#npm install .





# node (frontend)
# echo "install node"
# curl -sL https://deb.nodesource.com/setup_13.x | sudo -E bash -
# sudo apt-get install nodejs
# 
# npm add buttplug-node-websockets
