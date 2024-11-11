use microbit::hal::prelude::*;
use microbit::hal::uarte;
use microbit::hal::uarte::Baudrate;
use microbit::hal::uarte::Parity;

use crate::serial_setup::UartePort;

pub fn send_byte() {
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    nb::block!(serial.write(b'X')).unwrap();
    nb::block!(serial.write(b'Y')).unwrap();
    nb::block!(serial.write(b'Z')).unwrap();
    nb::block!(serial.flush()).unwrap();
}
