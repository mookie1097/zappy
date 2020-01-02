#!/usr/bin/env python3
"""Library for easilly controlling the relay board for the raspberry pi"""

# getting the main GPIO libraly
import RPi.GPIO as GPIO

# setting a current mode
GPIO.setmode(GPIO.BCM)
#removing the warings 
GPIO.setwarnings(False)

#creating a list (array) with the number of GPIO's that we use 
pins = [14,15,17,18] 
#setting the mode for all pins so all will be switched on 
GPIO.setup(pins, GPIO.OUT)

off = GPIO.HIGH
on = GPIO.LOW

def	debugprint(stringy):
	if(1):
		print(stringy)


#to reset them
def turnOffAll():
	debugprint(f"resetting all relays")
	for pin in reversed(pins) :
		GPIO.output(pin,  off)

#ensure everything is off when file is initized
turnOffAll()

print("relay control activated!")

def setRelayOn(number):
	"""number is one indexed"""
	debugprint(f"relay {number} on")
	GPIO.output(pins[number - 1], on)

def setRelayOff(number):
	"""number is one indexed"""
	debugprint(f"relay {number} off")
	GPIO.output(pins[number - 1], off)

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
	GPIO.cleanup()
	print("Shutdown All relays")

