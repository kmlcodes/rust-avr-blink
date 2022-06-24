#![no_std]
#![no_main]
use ruduino::cores::current::port;
use ruduino::{delay::delay_ms, Pin};

#[no_mangle]
pub extern "C" fn main() {
    let led_pin = port::B5; // use pin 13 as led pin
    led_pin::set_output(); // set pin as output
    loop {
        led_pin::set_high(); //turn on
        delay_ms(1000); // delay for 1 second
        led_pin::set_low(); //turn off
        delay_ms(1000); // delay for 1 second
    }
}
