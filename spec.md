# spec

## json

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