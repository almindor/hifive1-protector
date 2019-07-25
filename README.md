# hifive1-protector

Hifive1 + Hifive1-revB "bootloader" code to protect against bricking written in Rust.

## *WARNING*

The protector code is meant to prevent getting the hifive1 and hifive1-revB boards into an un-flashable state. This code is currently a work in progress.

To recover the official **HiFive1** bootloader see [this forum thread](https://forums.sifive.com/t/bootloader-restore/2429/18)

Unlike the official bootloader, the protector does not attempt to intialize any peripheral of the hifive1 board. This is to minimize chances of problems stemming from state mangling on boot.

*The digital PIN7 (GPIO 23) will be checked on each boot. If you need to use a different PIN modify the code as needed*

## Usage

The protector normally jumps to user code address specific to each board. If you need to "stop" on boot in order to flash a different user program short digital pin 7 (GPIO 23) to any ground (GND) and press the reset button. The protector will indicate being in "flash mode" by blinking the blue LED in an endless loop. Remove the short before resetting/uploading the new program to prevent going into "flash mode" on the next reset.

### HiFive1

`cargo run --release --features board-hifive1`

### HiFive1 Rev B

`cargo run --release --features board-hifive1-revb`
