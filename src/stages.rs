use ufmt::derive::uDebug;
use arduino_hal::port::{mode::Output, Pin, D9, D10, D11, D12};

use crate::metrics::{Direction, Power};

const FINAL_CODE: [u8; 4] = [
    2, 6, 4, 3
];

#[derive(uDebug, Clone, Copy)]
pub enum Stage {
    Start,
    Step1,
    Step2,
    Step3,
    Complete,
}

impl Stage {
    pub fn transition(self: Self, direction: Direction, power: Power) -> Self {
        match self {
            Stage::Start => {
                if direction == Direction::North {
                    Stage::Step1
                } else {
                    Stage::Start
                }
            },
            Stage::Step1 => {
                if direction == Direction::West {
                    Stage::Step2
                } else {
                    Stage::Start
                }
            },
            Stage::Step2 => {
                if direction == Direction::NorthEast {
                    Stage::Step3
                } else {
                    Stage::Start
                }
            }
            Stage::Step3 => {
                if direction == Direction::East {
                    Stage::Complete
                } else {
                    Stage::Start
                }
            }
            Stage::Complete => Stage::Complete
        }
    }
}

pub struct Leds {
    pub led0: Pin<Output, D9>,
    pub led1: Pin<Output, D10>,
    pub led2: Pin<Output, D11>,
    pub led3: Pin<Output, D12>,
}

impl Leds {
    fn reset(self: &mut Self) {
        self.led0.set_low();
        self.led1.set_low();
        self.led2.set_low();
        self.led3.set_low();
    }

    pub fn set(self: &mut Self, stage: Stage) {

        match stage {
            Stage::Start => self.reset(),
            Stage::Step1 => self.led0.set_high(),
            Stage::Step2 => self.led1.set_high(),
            Stage::Step3 => self.led2.set_high(),
            Stage::Complete => self.led3.set_high(),
        }
    }
}
