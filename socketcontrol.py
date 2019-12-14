#!/usr/bin/env python3
"""socket hookup for the js in index.html to call, uses pirelaycontrol"""

import asyncio
import websockets
import pirelaycontrol
import sys
import time

async def socket(websocket, path):
	while(True):
		print("listening")
		message = await websocket.recv()
		print(f"< {message}")
		if message == "off":
			pirelaycontrol.turnOffAll()
		messagge   = message.split()
		offOrOn    = int(message[0])
		intMessage = int(message[1])
		print(f"{intMessage}, {offOrOn}")
		pirelaycontrol.setRelay(intMessage, offOrOn)
		
		responce = f"recv {message}!"

		await websocket.send(responce) 
		print(f"> {responce}")

start_server = websockets.serve(socket, "0.0.0.0", 8765)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()


pirelaycontrol.cleanup()

