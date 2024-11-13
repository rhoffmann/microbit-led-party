use core::fmt::Write;
use core::str;
use heapless::Vec;
use lsm303agr::{AccelOutputDataRate, Lsm303agr};
use microbit::hal::uarte::{Baudrate, Parity};
use microbit::hal::{prelude::*, uarte};
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A, Board};
use nb::block;
use rtt_target::rprintln;

use crate::serial_setup::UartePort;

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

pub fn read_accelero_magneto() {
    let board = Board::take().unwrap();

    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut acc = [0];
    let mut mag = [0];

    i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc)
        .unwrap();
    i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag)
        .unwrap();

    rprintln!("The accelerometer chip's ID is: {:#b}", acc[0]);
    rprintln!("The magnetometer chip's ID is: {:#b}", mag[0]);
}

pub fn read_with_lsm303() {
    let board = Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
        }
    }
}

pub fn read_with_command() -> ! {
    let board = Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor
        .set_mag_odr(lsm303agr::MagOutputDataRate::Hz50)
        .unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        let mut buffer: Vec<u8, 32> = Vec::new();

        loop {
            let byte = block!(serial.read()).unwrap();
            if byte == 13 {
                break;
            }
            if buffer.push(byte).is_err() {
                write!(serial, "error: buffer full\r\n").unwrap();
                break;
            }
        }

        if str::from_utf8(&buffer).unwrap().trim() == "acc" {
            while !sensor.accel_status().unwrap().xyz_new_data {}
            let data = sensor.accel_data().unwrap();
            write!(
                serial,
                "Acceleration: x {} y {} z {}\r\n",
                data.x, data.y, data.z
            )
            .unwrap();
        } else if str::from_utf8(&buffer).unwrap().trim() == "mag" {
            while !sensor.mag_status().unwrap().xyz_new_data {}
            let data = sensor.mag_data().unwrap();
            write!(
                serial,
                "Magnetometer: x {} y {} z {}\r\n",
                data.x, data.y, data.z
            )
            .unwrap();
        } else {
            write!(serial, "Unknown command\r\n").unwrap();
        }
    }
}
