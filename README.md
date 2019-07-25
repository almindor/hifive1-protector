# hifive1-protector

Hifive1 + Hifive1-revB "bootloader" code to protect against bricking written in Rust.

## *WARNING*

The protector (some call it "bootloader") code is meant to prevent getting the hifive1 and hifive1-revB boards into an un-flashable state. This code is currently a work in progress. To recover the official Sifive1 bootloader for please see [this forum thread](https://forums.sifive.com/t/bootloader-restore/2429/18)

## Usage

This crate overrides the "bootloader" memory at `0x20000000` and writes the current version of the protector code. The protector normally jumps to user code address specific to each board. If you need to "stop" on boot in order to flash a different user program short digital pin 7 (GPIO 23) to any ground (GND) and press the reset button. The protector will indicate being in "flash mode" by blinking the blue LED in an endless loop.

### Hifive1

`cargo run --release --features board-hifive1`

### Hifive1-revB

`cargo run --release --features board-hifive1-revb`
