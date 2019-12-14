# zappy
run `bash setup` to setup the service

`sudo service zappyServer start`
`sudo service zappyServer restart`
`sudo service zappyServer stop`
`sudo service zappyServer status`

`pirelaycontrol.py` is a library for controlling the relay
`socketcontrol.py` uses pirelaycontrol to make a websocket to control it

`zappyServer` is an init.d file to use zappy as a service for easy setup
`updateservice.sh` can be used if edits are made to the zappyServer file 

`runbeacon.sh` uses `beacon.py` to run an eddystone beacon to know what ip:port to connect to
you can use https://play.google.com/store/apps/details?id=com.bridou_n.beaconscanner to view the beacon
