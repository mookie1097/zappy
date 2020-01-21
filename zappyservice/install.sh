sudo apt update
sudo apt install git


mkdir .ssh

scp /home/violet/.ssh/githubkey /home/violet/.ssh/config /home/violet/.ssh/authorized_keys pi@raspberrypi.escher.lan






python3 -m venv .env
source
pip3 install RPI.GPIO adafruit-blinka
