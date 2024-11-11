use core::fmt::Write;
use microbit::hal::prelude::*;
use microbit::hal::uarte;
use microbit::hal::uarte::Baudrate;
use microbit::hal::uarte::Parity;

use crate::serial_setup::UartePort;

pub fn send_bytes(bytes: &[u8]) {
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

    for byte in bytes.iter() {
        nb::block!(serial.write(*byte)).unwrap()
    }
    nb::block!(serial.flush()).unwrap();
}

pub fn write(text: &str) {
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

    write!(serial, "{}", text).unwrap();
}
