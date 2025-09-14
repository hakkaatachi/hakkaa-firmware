//! GPIO pin abstractions for switching things on an off whithout having to remember the actual
//! hardware setup behing it.

use esp_hal::gpio::Output;

/// Convenience wrapper for switching outputs (like LEDs) without having to remember the actual
/// hardware behind this task.
///
/// There is the crate switch-hal for that, but it does not support embedded-hal
/// 1.0 yet.
#[derive(Debug)]
pub struct LowActiveSwitch<'a> {
    inner: Output<'a>,
}

impl<'a> LowActiveSwitch<'a> {
    /// Creates a new `LowActiveSwitch` from the given output.
    pub fn new(output: Output<'a>) -> Self {
        Self { inner: output }
    }

    /// Turns the output on.
    pub fn switch_off(&mut self) {
        self.inner.set_high();
    }

    /// Turns the output off.
    pub fn switch_on(&mut self) {
        self.inner.set_low();
    }

    /// Sets the output to the supplied state.
    pub fn switch(&mut self, on: bool) {
        match on {
            true => self.switch_on(),
            false => self.switch_off(),
        }
    }
}
