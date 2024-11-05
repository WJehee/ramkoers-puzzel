#![no_std]
#![no_main]

use core::panic::PanicInfo;
use ufmt::derive::uDebug;

mod metrics;
use metrics::{Direction, Power};

#[derive(uDebug)]
enum Stage {
    Start,
    Step1,
    Step2,
    Step3,
    Complete,
}

impl Stage {
    fn transition(self: Self, direction: Direction, power: Power) -> Self {
        match self {
            Stage::Start => {
                if direction == Direction::North {
                    Stage::Step1
                } else {
                    Stage::Start
                }
            },
            Stage::Step1 => Stage::Start,
            s => s,
        }
    }

    fn leds(self: Self) {
        match self {
            Stage::Complete => loop {
                // TODO: show final sequence
            }
            // TODO: other LEDS
            s => {}
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let p = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(p);
    let mut adc = arduino_hal::Adc::new(p.ADC, Default::default());
    let mut serial = arduino_hal::default_serial!(p, pins, 115200);

    let mut led = pins.d13.into_output();
    let pot = pins.a0.into_analog_input(&mut adc);

    let mut stage = Stage::Start;
    let mut prev_direction = Direction::North;
    let mut prev_power = Power::Low;
    let mut steps = 0;

    loop {
        let value = pot.analog_read(&mut adc);
        let angle = map(value, 1023, 180);
        let direction: Direction = Direction::from_angle(angle);

        // TODO: make this use the sliding potmeter
        let value = 500;
        let speed = map(value, 1023, 100);
        let power = Power::from_speed(speed);

        // Reset timer if changed
        if direction != prev_direction || power != prev_power {
            prev_direction = direction;
            prev_power = power;
            steps = 0;
        } else {
            steps += 1;
        }

        ufmt::uwriteln!(serial, "\rangle: {} -> {:?}, power: {} -> {:?}", angle, direction, speed, power).unwrap();

        // After 5 seconds in the same stage, check for transition
        if steps == 5 {
            stage = stage.transition(direction, power);
            //stage.leds();
            steps = 0;
            ufmt::uwriteln!(serial, "\rstage: {:?}", stage).unwrap();
        }
        led.toggle();
        arduino_hal::delay_ms(1000);

        // TODO: make servo work
    }
}

/// Maps values from 1 range to another range, assumes the minimum is 0 for both
fn map(val: u16, max_original: u16, max_result: u16) -> u16 {
    let fraction = val as f32 / max_original as f32;
    (max_result as f32 * fraction) as u16
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

