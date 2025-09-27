//! Higher level abstractions for LEDs, like for cycling the storey LEDs.

use crate::switch::LowActiveSwitch;
use embassy_time::{Duration, Ticker};

/// The number of storey LEDs on the board.
pub const STOREY_LEDS: usize = 8;

/// Convenience wrapper proviving higher-level functionality for all the storey LEDs like for
/// example cycling one switched on led.
#[derive(Debug)]
pub struct Storeys<'a> {
    leds: [LowActiveSwitch<'a>; 8],
}

impl<'a> Storeys<'a> {
    pub fn new(leds: [LowActiveSwitch<'a>; 8]) -> Self {
        Self { leds }
    }

    pub fn free(self) -> [LowActiveSwitch<'a>; 8] {
        self.leds
    }

    /// Switches all storey LEDs off.
    pub fn all_off(&mut self) {
        log::debug!("Dn off");
        self.leds.iter_mut().for_each(|led| led.switch_off());
    }

    /// Switches all storey LEDs on.
    pub fn all_on(&mut self) {
        log::debug!("Dn on");
        self.leds.iter_mut().for_each(|led| led.switch_on());
    }

    /// Blinks all storey LEDs simultaneously.
    ///
    /// Blinking is performed until the returned future is dropped. So `await`ing this future alone
    /// will block forever. Use [`embassy_futures::select::select`] and friends to blink the LEDs
    /// while waiting for some other event to happen.
    pub async fn blink(&mut self, step: Duration) {
        let mut ticker = Ticker::every(step);

        loop {
            self.all_on();
            ticker.next().await;
            self.all_off();
            ticker.next().await;
        }
    }

    /// Cycles through all storey LEDs, switching on one at a time.
    ///
    /// Cycling is performed until the returned future is dropped. So `await`ing this future alone
    /// will block forever. Use [`embassy_futures::select::select`] and friends to cycle the LEDs
    /// while waiting for some other event to happen.
    pub async fn cycle(&mut self, step: Duration) {
        let mut ticker = Ticker::every(step);

        self.all_off();

        loop {
            for n in 0..self.leds.len() {
                log::debug!("cycle D{}", n + 1);

                let previous = (n + self.leds.len() - 1) % self.leds.len();
                self.leds[previous].switch_off();
                self.leds[n].switch_on();
                ticker.next().await;
            }
        }
    }

    /// Sets the storey LEDs to the supplied pattern. The bit at index _n_ specifies the output
    /// state of the LED at index _n_ from the array `leds` passed to [`Storeys::new`].
    pub fn set_pattern(&mut self, pattern: u8) {
        for i in 0..self.leds.len() {
            let bit = (pattern >> i) & 1;
            let on = bit == 1;
            self.leds[i].switch(on);
        }
    }
}
