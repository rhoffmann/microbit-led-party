#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

mod led_party;
mod serial_setup;
mod uart;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // uart::send_bytes(b"The quizck brown fox jumps over the lazy dog.");
    uart::write("Hello, World!");
    // led_party::led_loop()

    loop {}
}
