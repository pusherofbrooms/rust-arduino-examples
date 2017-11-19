Arduino Examples in rust
====

This is an attempt to exercise rust-avr with the simple examples included with the Arduino IDE.

https://github.com/avr-rust/rust has great instructions for setting up your avr-rust environment.

This repository includes a templating system mostly for my convenience, but hopefully, it will be useful to you as well. In order use this template system, you must have installed the avr-rust environment, and you must set

    export XARGO_RUST_SRC=LOCATION_OF_YOUR_AVR-RUST_BUILD

To create a new project, run

    ./tools/new.sh NAME

where NAME is the name of your new project. Study the build.sh and upload.sh scripts.
