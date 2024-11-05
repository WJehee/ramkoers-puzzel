#![no_std]
#![no_main]

use core::panic::PanicInfo;

use ufmt::derive::uDebug;

#[derive(uDebug, PartialEq)]
enum Direction {
    North,
    NorthWest,
    NorthEast,
    East,
    West,
}

impl Direction {
    fn from_angle(angle: u16) -> Self {
        if angle <= 36 {
            Direction::West
        } else if angle <= 72 {
            Direction::NorthWest
        } else if angle <= 108 {
            Direction::North
        } else if angle <= 144 {
            Direction::NorthEast
        } else {
            Direction::East
        }
    }
}

#[derive(uDebug)]
enum Stage {
    Start,
    Step1,
    Step2,
    Step3,
    Complete,
}

impl Stage {
    fn transition(self: Self, direction: Direction, power: u16) -> Self {
        match self {
            Stage::Start => {
                if direction == Direction::North && power == 100 {
                    Stage::Step1
                } else {
                    Stage::Start
                }
            },
            Stage::Step1 => Stage::Start,
            s => s,
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
    let pot = pins.a5.into_analog_input(&mut adc);

    let mut stage = Stage::Start;

    loop {
        let value = pot.analog_read(&mut adc);
        let angle = map(value, 1023, 180);
        let direction: Direction = Direction::from_angle(angle);

        // TODO: make this use the sliding potmeter
        let power = 100;

        ufmt::uwriteln!(serial, "\rangle: {} -> {:?}, power: {}", angle, direction, power).unwrap();
        stage = stage.transition(direction, power);
        ufmt::uwriteln!(serial, "\rstage: {:?}", stage).unwrap();

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

