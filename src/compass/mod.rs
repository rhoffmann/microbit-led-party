use core::f32::consts::PI;

use calibration::{calibrated_measurement, Calibration};
use led::{direction_to_led, Direction};
use libm::{atan2f, sqrtf};
use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate};
use microbit::display::blocking::Display;
use microbit::hal::prelude::*;
use microbit::{hal::twim, hal::Timer, pac::twim0::frequency::FREQUENCY_A, Board};
use rtt_target::rprintln;

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

        let theta = atan2f(data.y as f32, data.x as f32);

        let x = data.x as f32;
        let y = data.y as f32;
        let z = data.z as f32;

        let magnitude = sqrtf(x * x + y * y + z * z);

        let dir = if theta < -7. * PI / 8. {
            Direction::West
        } else if theta < -5. * PI / 8. {
            Direction::SouthWest
        } else if theta < -3. * PI / 8. {
            Direction::South
        } else if theta < -PI / 8. {
            Direction::SouthEast
        } else if theta < PI / 8. {
            Direction::East
        } else if theta < 3. * PI / 8. {
            Direction::NorthEast
        } else if theta < 5. * PI / 8. {
            Direction::North
        } else if theta < 7. * PI / 8. {
            Direction::NorthWest
        } else {
            Direction::West
        };

        rprintln!("{} nTesla, {} mGauss", magnitude, magnitude / 100.0);
        display.show(&mut timer, direction_to_led(dir), 100);
    }
}
