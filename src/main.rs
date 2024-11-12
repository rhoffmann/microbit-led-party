#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

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
    i2c::reat_acc_magneto();

    loop {}
}
