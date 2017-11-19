#!/usr/bin/env bash

# set the port to whatever your arduino board uses
PORT=/dev/ttyACM0

avrdude -c arduino -p atmega328p -P $PORT -U flash:w:target/avr-atmega328p/release/__NAME__.elf

