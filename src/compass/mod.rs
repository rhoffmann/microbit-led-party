use calibration::{calibrated_measurement, Calibration};
use led::{direction_to_led, Direction};
use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate};
use microbit::display::blocking::Display;
use microbit::hal::prelude::*;
use microbit::{hal::twim, hal::Timer, pac::twim0::frequency::FREQUENCY_A, Board};

pub mod calibration;
pub mod led;

pub fn run() -> ! {
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();

    let mut sensor = sensor.into_mag_continuous().ok().unwrap();
    let calibration = Calibration::default();

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data {}
        let mut data = sensor.mag_data().unwrap();
        data = calibrated_measurement(data, &calibration);

        let dir = match (data.x > 0, data.y > 0) {
            (true, true) => Direction::NorthEast,
            (false, true) => Direction::NorthWest,
            (false, false) => Direction::SouthWest,
            (true, false) => Direction::SouthEast,
        };

        display.show(&mut timer, direction_to_led(dir), 100);
    }
}
