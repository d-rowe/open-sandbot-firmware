## OpenSandbot
OpenSandbot is an open-source robotic kinetic art table. It has four components: parts, hardware, firmware, and app.

### Firmware
This firmware is written in rust for the $4 Raspberry Pi Pico (RP2040) and provides a UART API on [UART0](https://pico.pinout.xyz/) to control the sandbot.

### API
The API expects messages to be formatted as `METHOD[ ARG0][ ARG1]\n`. All messages (rx and tx) expect the new line character (`\n`) at the end of each message.

- Movement: Ex. `MOVE 1.2 0.5` would move to theta 1.2 rho 0.5.

The API also emits the status messages
- `STATUS IDLE`: the sandbot is idle and not currently moving
- `STATUS MOVING` the sandbot is currently moving to a target position
