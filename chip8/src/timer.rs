//! Timer-related structs and methods.

pub struct Timer {
    /// Delay timer for the CHIP-8.
    ///
    /// The delay timer is one byte and decremented by one 60 times per second if its value is > 0.
    pub delay_timer: u8,

    /// Sound timer for the CHIP-8.
    ///
    /// The sound timer is one byte and decremented by one 60 times per second if its value is > 0.
    pub sound_timer: u8,
}

impl Timer {
    pub fn new() -> Self {
        let delay_timer: u8 = 0;
        let sound_timer: u8 = 0;
        Timer {
            delay_timer,
            sound_timer,
        }
    }

    /// Checks if `delay_timer` and `sound_timer` are greater than 0.
    ///
    /// If the values are greater than 0, decrement them by one. Makes a sound if `sound_timer` is
    /// greater than 0.
    pub fn cycle(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.beep();
            self.sound_timer -= 1;
        }
    }

    fn beep(&self) {
        println!("BEEP");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle() {
        let mut timer = Timer::new();
        timer.delay_timer += 1;
        timer.sound_timer += 1;
        timer.sound_timer += 1;
        timer.cycle();
        timer.cycle();
        timer.cycle();
    }
}
