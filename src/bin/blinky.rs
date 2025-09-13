#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{DriveMode, Level, Output, OutputConfig, Pull};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use hakkaa::switch::LowActiveSwitch;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

fn delay(duration: Duration) {
    let start = Instant::now();
    while start.elapsed() < duration {}
}

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Current flows from the power source (+) through the LED, a current limiting resistor, and
    // finally into a GPIO pin with its low-side switching transistor to GND (-).
    let led_pin_config = OutputConfig::default()
        .with_drive_mode(DriveMode::OpenDrain)
        .with_pull(Pull::None);
    let mut d1 = LowActiveSwitch::new(Output::new(peripherals.GPIO3, Level::High, led_pin_config));
    let mut d2 = LowActiveSwitch::new(Output::new(peripherals.GPIO4, Level::High, led_pin_config));

    let t1 = Duration::from_millis(500);

    log::info!("Hello world!");

    loop {
        log::info!("D1");
        d1.switch_on();
        d2.switch_off();
        delay(t1);
        log::info!("D2");
        d2.switch_on();
        d1.switch_off();
        delay(t1);
    }
}
