#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use adjacent_pair_iterator::AdjacentPairIterator;
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Instant, Timer};
use esp_backtrace as _;
use esp_hal::gpio::Input;
use hakkaa::board::Board;
use hakkaa::led::Storeys;
use heapless::HistoryBuffer;

type DurationSignal = Signal<CriticalSectionRawMutex, Duration>;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// A symmetrical smile pattern. Every entry represents a row. Squint your eyes and have a look from
// the right side: :)
#[rustfmt::skip]
static SMILE_PATTERN: [u8; 26] = [
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b11000000,
    0b11000000,
    0b00010000,
    0b00001000,
    0b00000100,
    0b00000010,
    0b00000010,
    0b00000001,
    0b00000001,
    0b00000001,
    0b00000001,
    0b00000010,
    0b00000010,
    0b00000100,
    0b00001000,
    0b00010000,
    0b11000000,
    0b11000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
];

async fn delay(duration: Duration) {
    Timer::after(duration).await;
}

/// Determines the shake duration. This is the duration between the first falling edges of the
/// pulse trains generated when the shake sensor activates (you can hear its click sound).
#[embassy_executor::task]
async fn shake_period(mut sensor: Input<'static>, signal: &'static DurationSignal) {
    let debounce_delay = Duration::from_millis(150);
    let mut history = HistoryBuffer::<_, 6>::new();

    loop {
        // Wait for a falling edge. When shaking, this is likely the first one from an end
        // position.
        sensor.wait_for_falling_edge().await;

        // Record the current timestamp and store it into the history for computing an average
        // shake period from.
        let now = Instant::now();
        history.write(now);

        // Actually compute the average shake period and signal it to others (if there is one)
        let sum: Option<Duration> = history
            .oldest_ordered()
            .adjacent_pairs()
            .map(|(older, younger)| {
                let duration = younger.checked_duration_since(*older);
                duration.unwrap()
            })
            .reduce(|acc, duration| acc + duration);
        if let Some(sum) = sum {
            let count = history.len().checked_sub(1).unwrap().try_into().unwrap();
            let mean = sum / count;
            signal.signal(mean);
        }

        // The shake sensor will bounce (on and off) several times at when it activates. Try to
        // ignore bouncing after the first edge by just sitting tight and waiting.
        delay(debounce_delay).await;
    }
}

/// Display the pattern two times for shaking the board back and forth.
async fn display_pattern_back_and_forth<'a>(
    storeys: &mut Storeys<'a>,
    pattern: &[u8],
    period: Duration,
) {
    // Record when this pattern as a reference for timing the display of individual rows.
    let pattern_start = Instant::now();

    // Compute the duration of a single row.
    let half_period = period / 2;
    let row_duration = half_period / SMILE_PATTERN.len().try_into().unwrap();

    // Display the pattern two times: back and forth. So we are iterating over the rows of the
    // pattern on the way forward and one more time over the reversed pattern. Wrapping this into
    // an enumeration gives us a continuous index for each row which we can use to compute the
    // timing for displaying the actual row.
    for (index, row) in pattern.iter().chain(pattern.iter().rev()).enumerate() {
        // Update the storey LEDs for the current row.
        storeys.set_pattern(*row);

        // Delay for the remaining time for this row.
        let row: u32 = index.try_into().unwrap();
        let row_start = pattern_start + row * row_duration;
        let in_row = Instant::now().duration_since(row_start);
        let remaining = row_duration - in_row;
        delay(remaining).await;
    }
}

static SHAKE_PERIOD_SIGNAL: DurationSignal = DurationSignal::new();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // Initialize the board.
    let board = Board::init();

    log::info!("ハッカー the planet!");

    // Setup timing and storey LED abstraction. Let's start with a pretty long period for
    // cross-checking the pattern.
    let initial_period = Duration::from_secs(5);
    let mut period = initial_period;
    let mut storeys = Storeys::new(board.storey_leds);

    // Spawn a task for concurrently determining the shake period.
    spawner
        .spawn(shake_period(board.u2, &SHAKE_PERIOD_SIGNAL))
        .unwrap();

    loop {
        // Create the futures for cycling the pattern and waiting for the next shake signal. They
        // well be executed concurrently when awaiting them with select below.
        let pattern_future = display_pattern_back_and_forth(&mut storeys, &SMILE_PATTERN, period);
        let period_future = SHAKE_PERIOD_SIGNAL.wait();

        // Wait for whichever future gets ready first.
        match select(pattern_future, period_future).await {
            // We are done with displaying the pattern. Just continue with the next round.
            Either::First(_) => {}
            // We've got a new shake period from waiting for the shake period signal. This is
            // expected tho happen at an end position for the pattern. Displaying the pattern will
            // no longer be continued. Compute a new row duration and continue with the next round.
            Either::Second(new_period) => {
                period = new_period;
                log::info!("period: {} ms", period.as_millis());
            }
        }
    }
}
