# spec

## json


generic Command
```json
{
    "version":0.02,
    "id": 00000,
    "command": "cancel | trigger | ack?",
    "deviceId":1,
    "outputId":1,
    "power":304, //arbetrary, dependent on what its controlling
    "length": 10, //ms
    "error": "meow"
}
```

Trigger
```json
{
    "version":0.02,
    "id": 00000,
    "command": "trigger",
    "deviceId":1,
    "outputId":1,
    "power":304, //arbetrary, dependent on what its controlling
    "length": 10 //ms
}
```


Cancel
```json
{
    "version":0.02,
    "id": 00000,
    "command": "cancel",
    "deviceId":1,
    "outputId":1,
}
```

ack
```json
{
    "version":0.02,
    "id": 00000,
    "command": "ack",
    "error": "fuckies!",
}
```


## MEOW???
Send to client
```json
{
    "version": 0.02,
    "devices": [
        {
            "name": "power box",
            "id": 1,
            "channels": [ //electrical
                {
                    "id": 1,
                    "steps": 2048 //set by limit in config
                }
            ], 
            "outputs":[//physical
                {
                    "id": 1,
                    "channel": 1,
                    "dual": true,
                },
                {
                    "id": 2,
                    "channel": 1,
                    "dual": true,
                },
            ]
        }
    ]

}
```

config
```json
{
    "version": 0.02,
    "devices": [
        {
            "id": 1,
            "name": "powerbox",
            "channels": [
                {
                    "id": 1, //output 1
                    "pins": {
                        "pos": 1,
                        "neg": 2
                    }
                },
                {
                    "id": 2, //output 2
                    "pins": {
                        "pos": 3,
                        "neg": 4
                    }
                }
            ],
            "powerchangy": [
                {
                    "fastswitching": true,
                    "pin": 5,
                    "multiplier": 100, //this is a 100x larger pot, changes much more dirastically per step
                    "step": 256,
                    "max": 100, //in ohms? idk
                    "min": 0,
                    "limit": 199 //for saftey or smth
                },
                {
                    "fastswitching": true,
                    "pin": 6,
                    "multiplier": 1, //fine adjustment
                    "step": 256,
                    "max": 100, //idk!!
                    "min": 0, //probably dont need it!!!
                    "limit": 256
                }
            ]
        }
    ]

}
```
### old shit dont pay attention to it

types are `channel`, `relay`, and `program`

```json
{
    "type": "channel",
    "time": 100,      // milliseconds
    "intensity": 100, // percentage
    "state": true     // bool on/off

}
```
```json
{
    "type": "relay",
    "time": 100,  // milliseconds
    "state": true // bool on/off
}
```
```json
{
    "type": "program",
    "program": 1,  // program index number 
    "loop": true // bool on/off
}
```