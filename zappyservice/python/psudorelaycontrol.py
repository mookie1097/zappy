#!/usr/bin/env python3
"""FAKE Library for easilly controlling the relay board for the raspberry pi

DOES NOT DO ANYTHING
"""

def	debugprint(stringy):
	if(1):
		print("psudo",stringy)


#to reset them
def turnOffAll():
	debugprint(f"resetting all relays")


print("psudo relay control activated!")

#ensure everything is off when file is initized
turnOffAll()

def setRelayOn(number):
	"""number is one indexed"""
	debugprint(f"relay {number} on")

def setRelayOff(number):
	"""number is one indexed"""
	debugprint(f"relay {number} off")

def setRelay(number, onOff):
	"""number is one indexed, onOff is a bool"""
	if onOff == 1:
		setRelayOn(number)
	else:
		setRelayOff(number)

def cleanup():
	"""cleaning all GPIO's"""
	debugprint(f"cleaning up")
	turnOffAll()
	print("psudo Shutdown All relays")

