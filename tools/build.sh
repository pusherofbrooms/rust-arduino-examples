#!/usr/bin/env bash

if [ -z "XARGO_RUST_SRC" ]
then
    echo "please set XARGO_RUST_SRC to the location of your"
    echo "rust-avr build directory"
    exit 1
fi

rustup run avr-toolchain xargo build --target avr-atmega328p --release
