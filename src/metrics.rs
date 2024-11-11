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
