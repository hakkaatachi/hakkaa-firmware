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
use hakkaa::led::Storeys;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// A symmetrical heart pattern. Every entry represents a row. Squint your eyes and have a look from
// the right side: :)
#[rustfmt::skip]
static SMILE_PATTERN: [u8; 16] = [
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b01110000,
    0b11111100,
    0b11111110,
    0b01111111,
    0b01111111,
    0b11111110,
    0b11111100,
    0b01110000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
];

async fn delay(duration: Duration) {
    Timer::after(duration).await;
}

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    // Intitialize the board.
    let board = Board::init();

    // Setup timing and storey LED abstradtion.
    let row_delay = Duration::from_millis(5);
    let image_delay = Duration::from_millis(30);
    let mut storeys = Storeys::new(board.storey_leds);

    log::info!("愛 the planet!");

    // Display the pattern one row after another. We are dealing with a symmetrical pattern and
    // don't have to pay attention to the direction of movement.
    loop {
        for pattern in SMILE_PATTERN.iter() {
            storeys.set_pattern(*pattern);
            delay(row_delay).await;
        }

        delay(image_delay).await;
    }
}
