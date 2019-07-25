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
    
    // get "go to flash mode" DIG7 (GPIO pin 23)
    let flash_mode_pin = pin!(pins, dig7).into_pull_up_input();

    // if we shorted DIG7 (GPIO pin 23) to GND we want to go to "flash mode"
    if flash_mode_pin.is_low().unwrap() {
        // get blue LED pin
        let mut blue_led = pin!(pins, led_blue).into_inverted_output();
        // set up sleeping
        let mut sleep = Sleep::new(dr.core_peripherals.clint.mtimecmp, clocks);

        // loop endlessly blinking blue led so program can be uploaded
        loop {
            blue_led.on();
            sleep.delay_ms(500u32);
            blue_led.off();
            sleep.delay_ms(500u32);
        }
    }

    // return DIG7 (GPIO pin 23) to default state
    flash_mode_pin.into_floating_input();

    // otherwise we jump to user code as usual
    const FINAL_ADDRESS: usize = 0x20010000;
    let user_main: fn() -> ! = unsafe { mem::transmute(FINAL_ADDRESS) };
    user_main();
}
