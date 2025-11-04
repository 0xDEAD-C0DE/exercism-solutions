use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,

}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}:{}{}",
        if self.hours >= 0 && self.hours <= 9 { "0"}
            else { "" }, self.hours, 
        if self.minutes >= 0 && self.minutes <= 9{ "0" }
            else { "" }, self.minutes
        ) 
    }
    
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
    let mut hours = hours;
     
    hours = hours + (minutes / 60);
    if minutes.is_negative(){
        if minutes.rem_euclid(60) > 0 && minutes.rem_euclid(60) < 60 {
            hours -= 1;
        }
    }
    
        Self {
            hours: hours.rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hrs = self.hours;
            if minutes.is_negative(){
                hrs += (self.minutes + minutes).div_euclid(60);
            } else {
                hrs += (self.minutes + minutes) / 60;  
            }
    
    Self { 
        hours: hrs.rem_euclid(24), 
        minutes: (self.minutes + minutes).rem_euclid(60), 
        } 
    }
}
