#![no_std]
#![no_main]

use core::panic::PanicInfo;

use arduino_hal::{hal::port::Dynamic, port::{mode::Analog, Pin}, Adc};

#[arduino_hal::entry]
fn main() -> ! {
    let p = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(p);
    let mut serial = arduino_hal::default_serial!(p, pins, 115200);

    let mut led = pins.d13.into_output();

    //let mut adc = arduino_hal::Adc::new(p.ADC, Default::default());
    //let pot = pins.a5.into_analog_input(&mut adc);

    //let pot = Pot {
    //    pot,
    //    adc,
    //};

    loop {
        ufmt::uwriteln!(serial, "\rangle: {}", 10).unwrap();
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}

//struct Pot {
//    pot: Pin<Analog, Dynamic>,
//    adc: Adc,
//}
//
//impl Pot {
//    fn angle(mut self: Self) -> u16 {
//        let value = self.pot.analog_read(&mut self.adc);
//        let angle = value;
//        angle
//    }
//}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

