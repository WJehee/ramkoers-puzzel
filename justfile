path := "target/avr-atmega328p/release/embedded.elf"
device := "/dev/ttyACM0"

_default:
    just --list

build:
    cargo build --release

run:
    cargo run --release

flash:
    avrdude -p m328p -c arduino -P {{device}} -b 115200 -U flash:w:{{path}}

