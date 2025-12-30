#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use hakkaa::board::Board;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

async fn delay(duration: Duration) {
    Timer::after(duration).await;
}

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    let board = Board::init();

    let t1 = Duration::from_millis(500);
    let [mut d1, mut d2, ..] = board.storey_leds;

    log::info!("Hello world!");

    loop {
        log::info!("D1");
        d1.switch_on();
        d2.switch_off();
        delay(t1).await;
        log::info!("D2");
        d2.switch_on();
        d1.switch_off();
        delay(t1).await;
    }
}
