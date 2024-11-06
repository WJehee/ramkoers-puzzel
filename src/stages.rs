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
    counts: [u8; 4],
    blink: bool,
}

impl Leds {
    pub fn new(
        led0: Pin<Output, D9>,
        led1: Pin<Output, D10>,
        led2: Pin<Output, D11>,
        led3: Pin<Output, D12>,
    ) -> Self {
        Leds {
            led0, led1, led2, led3,
            counts: [0; 4],
            blink: false,
        }
    }
    
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
            Stage::Complete => {
                if self.blink {
                    let mut done = true;
                    if self.counts[0] < FINAL_CODE[0] { self.led0.set_high(); done = false; }
                    if self.counts[1] < FINAL_CODE[1] { self.led1.set_high(); done = false; }
                    if self.counts[2] < FINAL_CODE[2] { self.led2.set_high(); done = false; }
                    if self.counts[3] < FINAL_CODE[3] { self.led3.set_high(); done = false; }
                    for count in &mut self.counts {
                        *count = *count + 1;
                    }
                    if done {
                        for count in &mut self.counts {
                            *count = 0;
                        }
                    }
                } else {
                    self.reset();
                }
                self.blink = !self.blink;
            },
        }
    }
}