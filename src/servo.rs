use arduino_hal::port::{mode::Output, Pin, D9};

use crate::map;

pub struct Servo {
    pub pin: Pin<Output, D9>,
}

impl Servo {
    pub fn write_angle(self: &mut Servo, angle: u16) {
        let duty = map(angle, 0, 180, 100, 612);
        for _ in 0..50 {
            self.pin.set_high();
            arduino_hal::delay_us((1000 + duty).into());
            self.pin.set_low();
            arduino_hal::delay_us((19000 - duty).into());
        }
    }
}
