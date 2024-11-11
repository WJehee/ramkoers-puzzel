use arduino_hal::port::{mode::Output, Pin, D9};

use crate::map;

pub struct Servo {
    pub pin: Pin<Output, D9>,
}

impl Servo {
    pub fn write_angle(self: &mut Servo, angle: u16) {
        let duty = map(angle, 0, 180, 0, 2000);
        for _ in 0..10 {
            self.pin.set_high();
            arduino_hal::delay_us((0 + duty).into());
            self.pin.set_low();
            arduino_hal::delay_us((20000 - duty).into());
        }
    }
}
