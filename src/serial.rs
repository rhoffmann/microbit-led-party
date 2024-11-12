use core::fmt::Write;
use heapless::Vec;
use microbit::hal::prelude::*;
use microbit::hal::uarte;
use microbit::hal::uarte::Baudrate;
use microbit::hal::uarte::Parity;
use microbit::pac::UARTE0;
use rtt_target::rprint;
use rtt_target::rprintln;

use crate::serial_setup::UartePort;

pub struct Serial {
    port: UartePort<UARTE0>,
    buffer: Vec<u8, 32>,
}

impl Serial {
    pub fn new() -> Self {
        let board = microbit::Board::take().unwrap();

        let port = {
            let serial = uarte::Uarte::new(
                board.UARTE0,
                board.uart.into(),
                Parity::EXCLUDED,
                Baudrate::BAUD115200,
            );
            UartePort::new(serial)
        };

        Serial {
            port,
            buffer: Vec::new(),
        }
    }

    pub fn echo_server(&mut self) -> ! {
        loop {
            let byte = nb::block!(self.port.read()).unwrap();
            nb::block!(self.port.write(byte)).unwrap();
            rprint!("{}", byte as char);
        }
    }

    pub fn send_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            nb::block!(self.port.write(*byte)).unwrap()
        }
        nb::block!(self.port.flush()).unwrap();
    }

    pub fn write(&mut self, text: &str) {
        write!(self.port, "{}", text).unwrap();
    }

    pub fn reverse_string(&mut self) -> ! {
        loop {
            self.buffer.clear();
            loop {
                let byte = nb::block!(self.port.read()).unwrap();

                if self.buffer.push(byte).is_err() {
                    write!(self.port, "error: buffer full\r\n").unwrap();
                    break;
                }

                if byte == 13 {
                    for byte in self.buffer.iter().rev().chain(&[b'\n', b'\r']) {
                        nb::block!(self.port.write(*byte)).unwrap();
                    }
                    break;
                }
            }
            nb::block!(self.port.flush()).unwrap();
        }
    }
}
