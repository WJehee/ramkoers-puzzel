#![no_std]
#![no_main]

mod metrics;
mod stages;
mod servo;

use core::panic::PanicInfo;
use servo::Servo;
use stages::{Stage, Leds};
use metrics::{Direction, Power};

#[arduino_hal::entry]
fn main() -> ! {
    let p = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(p);
    let mut adc = arduino_hal::Adc::new(p.ADC, Default::default());
    let mut serial = arduino_hal::default_serial!(p, pins, 115200);

    let steering = pins.a0.into_analog_input(&mut adc);
    let slider = pins.a1.into_analog_input(&mut adc);
    let servo = Servo::new(p.TC1);

    let mut leds = Leds::new(
        pins.d10.into_output(),
        pins.d11.into_output(),
        pins.d12.into_output(),
        pins.d13.into_output(),
    );

    let mut stage = Stage::Start;
    let mut prev_direction = Direction::North;
    let mut prev_power = Power::Low;
    let mut steps = 0;

    loop {
        let value = steering.analog_read(&mut adc);
        let angle = map(value, 0, 1023, 0, 180);
        let direction: Direction = Direction::from_angle(angle);
        servo.write_angle(angle);

        let value = slider.analog_read(&mut adc);
        let speed = map(value, 0, 1023, 0, 100);
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
            steps = 0;
            ufmt::uwriteln!(serial, "\rstage: {:?}", stage).unwrap();
        }

        leds.set(stage);
        //ufmt::uwriteln!(serial, "{:?} {:?} {:?} {:?}",
        //    leds.led0.is_set_high(),
        //    leds.led1.is_set_high(),
        //    leds.led2.is_set_high(),
        //    leds.led3.is_set_high(),
        //).unwrap();

        arduino_hal::delay_ms(1000);
    }
}

pub fn map(mut value: u16, min_input: u16, max_input: u16, min_output: u16, max_output: u16) -> u16 {
    let input_range = max_input - min_input;
    let output_range = max_output - min_output;

    value -= min_input;
    let fraction = (value as f32) / (input_range as f32);
    
    let output = (output_range as f32 * fraction) as u16;
    output + min_output
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

