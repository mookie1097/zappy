#!/usr/bin/env python3
"""socket hookup for the js in index.html to call, uses pirelaycontrol"""

import asyncio
import websockets
#from pirelaycontrol import *
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
		j = json.loads(message)
		print("json:", j)
		
		setRelay(j['name'], j['state'])
		
		print(j['name'], j['state'])

		responce = f"recv {message}!"
		await websocket.send(responce) 
		print(f"> {responce}")

start_server = websockets.serve(socket, "0.0.0.0", 8765)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()


cleanup()

