use arduino_hal::pac::TC1;

use crate::map;

pub struct Servo {
    tc1: TC1,
}

impl Servo {
    pub fn new(
        tc1: TC1, 
    ) -> Self {
        // setup TC1 so that it has a 50Hz signal.
        // oc1a is connected to the control wire of the servo
        // 50Hz is achieved by dividing the 16Mhz clock source
        // of the arduino by the 64 prescaler and then again by 5000 ( which is the TOP value set via icr1 )
        tc1.tccr1a.write(|w| w.wgm1().bits(0b10).com1a().match_clear().com1b().match_clear());
        tc1.tccr1b.write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
        tc1.icr1.write(|w| w.bits(4999));

        Servo{tc1}
    }

    pub fn write_angle(&self, angle: u16) -> u16 {
        let duty = map(angle, 0, 180, 100, 612);
        self.tc1.ocr1a.write(|w| w.bits(duty));
        duty
    }
}
