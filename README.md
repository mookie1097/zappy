# zappy
run `bash setup` to setup the service and you're all set! that's it! it starts on startup and everything!

`sudo service zappyServer [start|restart|stop|status]`

`pirelaycontrol.py` is a library for controlling the relay  
This can be used standalone for simple relay control.  
`socketcontrol.py` uses pirelaycontrol to make a websocket to control the four relays I used.

`zappyServer` is an init.d file to use zappy as a service for easy setup  
`updateservice.sh` can be used if edits are made to the zappyServer file

`runbeacon.sh` uses `beacon.py` to run an eddystone beacon to know what ip:port to connect to  
you can use https://play.google.com/store/apps/details?id=com.bridou_n.beaconscanner to view the beacon  
This is so that when the ip changes it will be easy to find.
