echo "Cloning repo"
apt-get update
apt install git
# git clone https://github.com/violetbp/zappy.git
cd zappy
echo "installing"
bash serviceInstall.sh
