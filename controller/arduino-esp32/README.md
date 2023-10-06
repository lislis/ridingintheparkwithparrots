# Controller

## Components used

- small breadboard (optional if you connect wires directly)
- 4 cables (either for direct wiring or a breadboard)
- 1 MPU-6050 Gyroscope Accelerometer
- 1 esp32 board
- 1 usb cable to power the board (either on your laptop or a powerbank or so)
- something bikehandle like to attach it to (I used an actual bike handle)

## Wiring

| MPU-6050 | esp32 |
|-----|----|
| VCC | 3.3V |
| GND | GND |
| SCL | D22 |
| SDA | D21 |

## Variables

The esp32 needs to log into a wifi to make its measurements available for the game to read.

Therefore you need to adjust the `ssid` and `password` variables in the code.

Then flash it on the board!
