use lsm303agr::{AccelOutputDataRate, AccelScale, Lsm303agr};
use microbit::hal::prelude::*;
use microbit::hal::timer::Timer;
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A, Board};
use nb::Error;
use rtt_target::rprintln;

pub fn punch_o_meter() -> ! {
    const THRESHOLD: f32 = 0.5;

    let board = Board::take().unwrap();
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut countdown = Timer::new(board.TIMER0);
    let mut delay = Timer::new(board.TIMER1);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    // allow to measure up to 16G
    sensor.set_accel_scale(AccelScale::G16).unwrap();

    let mut max_g = 0.;
    let mut measuring = false;

    loop {
        while !sensor.accel_status().unwrap().xyz_new_data {}

        let g_x = sensor.accel_data().unwrap().x as f32 / 1000.0;

        if measuring {
            match countdown.wait() {
                Err(Error::WouldBlock) => {
                    if g_x > max_g {
                        max_g = g_x;
                    }
                }
                Ok(_) => {
                    rprintln!("Max acceleration: {}g", max_g);
                    max_g = 0.;
                    measuring = false;
                }
                Err(Error::Other(_)) => {
                    unreachable!()
                }
            }
        } else if g_x > THRESHOLD {
            rprintln!("START!");
            measuring = true;
            max_g = g_x;
            countdown.start(1_000_000_u32);
        }

        delay.delay_ms(20_u8);
    }
}
