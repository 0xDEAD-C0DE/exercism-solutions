use std::fmt;
const HOURS:i32 = 60;
const DAY:i32 = 1440; 

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes.div_euclid(HOURS), self.minutes.rem_euclid(HOURS))
    }
    
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: ((hours * HOURS) + minutes).rem_euclid(DAY),//+ DAY.rem_euclid(24),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0,  self.minutes + minutes)
    }
}