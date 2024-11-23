#![deny(unused_unsafe)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

pub mod compass;
mod gravity;
mod i2c;
mod led_party;
mod serial;
mod serial_setup;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // let mut serial = serial::Serial::new();
    // serial.send_bytes(b"Hello");
    // serial.echo_server();
    // serial.reverse_string();
    // led_party::led_loop()
    // i2c::read_accelero_magneto();
    // i2c::read_accel_loop();
    // i2c::read_magneto_loop();
    // i2c::read_with_command();
    // compass::run();
    gravity::punch_o_meter()
    // loop {}
}
