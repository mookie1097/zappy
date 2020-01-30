#!/usr/bin/env python3
"""socket hookup for the js in index.html to call, uses pirelaycontrol"""

import asyncio
import websockets
#from pirelaycontrol import setRelay, cleanup, turnOffAll
from psudorelaycontrol import setRelay, cleanup, turnOffAll
import sys
import time, json

async def socket(websocket, path):
	while(True):
		print("listening")
		message = await websocket.recv()
		print(f"message in: {message}")
		if message == "off":
			turnOffAll()
		
		messagge   = message.split()
		offOrOn    = int(message[0])
		intMessage = int(message[1])
		print(f"{intMessage}, {offOrOn}")
		pirelaycontrol.setRelay(intMessage, offOrOn)
		
		responce = f"recv {message}!"

		await websocket.send(responce) 
		print(f"> {responce}")
		# j = json.loads(message)
		# print("json:", j)
		
		# setRelay(j['name'], j['state'])
		
		# print(j['name'], j['state'])

		# responce = f"recv {message}!"
		# await websocket.send(responce) 
		# print(f"> {responce}")

start_server = websockets.serve(socket, "0.0.0.0", 8765)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()


cleanup()

