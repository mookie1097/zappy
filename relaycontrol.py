#!/usr/bin/env python3

import asyncio
import websockets


# getting the main GPIO libraly
import RPi.GPIO as GPIO
# getting the time libraly
import time
import sched
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

def turnOffAll():
	for pin in reversed(pins) :
		GPIO.output(pin,  off)

turnOffAll()

print("activated!")



async def socket(websocket, path):
	while(True):
		message = await websocket.recv()
		print(f"< {message}")
		if message == "off":
			turnOffAll()
		messagge = message.split()
		offoron  = int(message[0])
		intmessage=int(message[1])
		if offoron == 1:
			GPIO.output(pins[intmessage - 1], on)
		else:
			GPIO.output(pins[intmessage - 1], off)
			

		responce = f"recv {message}!"

		await websocket.send(responce)
		print(f"> {responce}")

start_server = websockets.serve(socket, "10.0.0.108", 8765)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()





#for loop where pin = 18 next 17 ,15, 14
for i in range(0,10):
	for pin in pins :
		#setting the GPIO to HIGH or 1 or true
		GPIO.output(pin,  GPIO.HIGH)
		#wait 0,5 second
		time.sleep(0.5)
		#setting the GPIO to LOW or 0 or false
		GPIO.output(pin,  GPIO.LOW)
		#wait 0,5 second
		time.sleep(0.5)

		#Checking if the current relay is running and printing it
	if not GPIO.input(pin):
		print("Pin "+str(pin)+" is working")


#same but the difference is that  we have 
#for loop where pin = 14 next 15,17,18
# backwards
for pin in reversed(pins) :
	GPIO.output(pin,  GPIO.LOW)

time.sleep(0.5)


#cleaning all GPIO's 
GPIO.cleanup()
print("Shutdown All relays")

