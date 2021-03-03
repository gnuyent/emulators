//! Display-related structs and methods.

/// Display for the CHIP-8.
///
/// The display is 64 pixels wide and 32 pixels tall. Since each pixel is either on or off, it
/// is stored as a bool.
///
/// TODO: Determine display update rate (Hz).
pub struct Display {
    pub screen: [[bool; 64]; 32],
}

impl Display {
    /// Creates a new, empty `Display`.
    pub fn new() -> Self {
        Display {
            screen: [[false; 64]; 32],
        }
    }

    /// Clears the screen.
    ///
    /// # Examples
    /// ```
    /// let mut display: Display = Display::new();
    /// // Turn on some pixels
    /// display.screen[5][5] = true;
    /// display.screen[36][12] = true;
    /// display.screen[61][27] = true;
    ///
    /// display.clear();
    /// // All pixels are now off
    /// ```
    pub fn clear(&mut self) {
        self.screen = [[false; 64]; 32];
    }
}
