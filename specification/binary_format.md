# Robot -> App

## search_app
The app sends a detection request to the app side.

```
0...............8...............16.............24. ............. 32
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Header ID     | Session ID    |          robot ipv4 addr                         
+---------------+---------------+---------------+---------------+
                                |      robot ipv4 ip port       |
+---------------+---------------+---------------+---------------+
|                          robot name
+---------------+---------------+---------------+---------------+
                           robot name
+---------------+---------------+---------------+---------------+
                           robot name
+---------------+---------------+---------------+---------------+
                           robot name                           |
+---------------+---------------+---------------+---------------+
```

parameter
- Header ID : `0xC9` (201)
- Session ID : `uint8`
- robot ipv4 addr :
    ```
    {
        uint8 octet[0]
        uint8 octet[1]
        uint8 octet[2]
        uint8 octet[3]
    }
    ```
- robot ipv4 ip port : `uint16`
- robot name : `uint8[16]`


## ping_request
Confirming the existence of the robot.
```
0...............8...............16
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Header ID     |  Session ID   |
+---------------+---------------+
```

parameter
- Header ID : `0xCA` (202)
- Session ID : `uint8`


# App -> Robot

## search_app_response
Confirming the existence of the robot.
```
0...............8...............16
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Header ID     |  Session ID   |
+---------------+---------------+
```

parameter
- Header ID : `0xCB` (203)
- Session ID : `uint8`

## ping_response
```
0...............8...............16
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Header ID     |  Session ID   |
+---------------+---------------+
```

parameter
- Header ID : `0xCC` (204)
- Session ID : `uint8`

## gamepad_value
Send to gamepad value
```
{
    {
        float left_joystic.x
        float left_joystic.y
        bool left_joystic.thumbstick_button
    }
    {
        float right_joystic.x
        float right_joystic.y
        bool right_joystic.thumbstick_button
    }
    {
        float left_trigger.value
        bool left_trigger.button
    }
    {
        float right_trigger.value
        bool right_trigger.button
    }
    {
        bool dbad.up
        bool dbad.down
        bool dbad.left
        bool dbad.right
    }
    {
        bool button.x
        bool button.y
        bool button.a
        bool button.b
    }
    {
        bool left_shoulder_button
        bool right_shoulder_button
    }
}

```

parameter
- Header ID : `0xCD` (205)

<!-- 
## templete
Confirming the existence of the robot.
```
0...............8...............16.............24. ............. 32
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Header ID     |                                               |
+---------------+---------------+---------------+---------------+
| unsigned long low |                                           |
+---------------+---------------+---------------+---------------+
```

parameter
- Header ID : `0x2F` (47) -->