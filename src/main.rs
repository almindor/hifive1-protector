#![no_std]
#![no_main]

extern crate panic_halt;

use core::mem;
use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::hal::delay::Sleep;
use hifive1::{Led, pin};

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());
    
    // get LED pins
    let mut blue_led = pin!(pins, led_blue).into_inverted_output();

    let mut sleep = Sleep::new(dr.core_peripherals.clint.mtimecmp, clocks);

    for _ in 0..9 {
        blue_led.on();
        sleep.delay_ms(500u32);
        blue_led.off();
        sleep.delay_ms(500u32);
    }

    const FINAL_ADDRESS: usize = 0x20010000;
    let user_main: fn() -> ! = unsafe { mem::transmute(FINAL_ADDRESS) };
    user_main();
}
