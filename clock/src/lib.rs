#[derive(Debug, PartialEq)]
pub struct Clock {
    h: i32,
    m: i32,
}

impl Clock {
    fn modular(a: i32, b: i32) -> i32 {
        ((a % b) + b) % b
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut c = Clock {
            h: hours + minutes / 60,
            m: minutes % 60,
        };

        if c.m < 0 {
            c.h -= 1;
        }
        c.h = Clock::modular(c.h, 24);
        c.m = Clock::modular(c.m, 60);
        c
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.h, self.m + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.h, self.m).to_string()
    }
}
