use ufmt::derive::uDebug;

#[derive(uDebug, PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    NorthWest,
    NorthEast,
    East,
    West,
}

impl Direction {
    pub fn from_angle(angle: u16) -> Self {
        if angle <= 36 {
            Direction::East
        } else if angle <= 72 {
            Direction::NorthEast
        } else if angle <= 108 {
            Direction::North
        } else if angle <= 144 {
            Direction::NorthWest
        } else {
            Direction::West
        }
    }
}


#[derive(uDebug, PartialEq, Copy, Clone)]
pub enum Power {
    Low,
    Half,
    Full,
}

impl Power {
    pub fn from_speed(speed: u16) -> Self {
        if speed <= 25 {
            Power::Low
        } else if speed <= 75 {
            Power::Half
        } else {
            Power::Full
        }
    }
}
