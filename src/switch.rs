use esp_hal::gpio::Output;

// Some convenience for switching LEDs without having to remember the actual hardware behind this
// task. There is the crate switch-hal for that, but it does not support embedded-hal 1.0 yet.
pub struct LowActiveSwitch<'a> {
    inner: Output<'a>,
}

impl<'a> LowActiveSwitch<'a> {
    pub fn new(output: Output<'a>) -> Self {
        Self { inner: output }
    }

    pub fn switch_off(&mut self) {
        self.inner.set_high();
    }

    pub fn switch_on(&mut self) {
        self.inner.set_low();
    }
}
