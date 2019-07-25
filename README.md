# hifive1-protector

Hifive1 + Hifive1-revB "bootloader" code to protect against bricking written in Rust.

## *WARNING*

The protector (some call it "bootloader") code is meant to prevent getting the hifive1 and hifive1-revB boards into an un-flashable state. This code is currently a work in progress. To recover the official Sifive1 bootloader for please see [this forum thread](https://forums.sifive.com/t/bootloader-restore/2429/18)

## Usage

This crate overrides the "bootloader" memory at `0x20000000` and writes the current version of the protector code. The code as of this version just waits for 10s blinking the blue on-board LED before jumping to the user program code address for the given board. This allows restoring a program in the span of the wait period should the user program invalidate the CPU for uploading.

Future versions will probably mimic the original behavior via reset button state switching.

### Hifive1

`cargo run --release --features board-hifive1`

### Hifive1-revB

`cargo run --release --features board-hifive1-revb`
