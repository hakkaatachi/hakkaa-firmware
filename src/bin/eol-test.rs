#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{DriveMode, Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::timer::systimer::SystemTimer;

use hakkaa::board::Storeys;
use hakkaa::switch::LowActiveSwitch;

extern crate alloc;

type ButtonSignal = Signal<CriticalSectionRawMutex, ()>;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

/// A convenience wrapper providing a simple delay.
async fn delay(duration: Duration) {
    Timer::after(duration).await;
}

/// Waits for a single press of `button` with input debouncing.
async fn wait_for_button<'a>(button: &mut Input<'a>) {
    let debounce_delay = Duration::from_millis(100);
    log::info!("waiting for switch");

    log::info!("waiting for high");
    button.wait_for_high().await;
    delay(debounce_delay).await;
    log::info!("waiting for low");
    button.wait_for_low().await;
    delay(debounce_delay).await;
    log::info!("waiting for high again");
    button.wait_for_high().await;
}

/// Waits for `n` presses of `button`.
async fn wait_for_button_n_times<'a>(button: &mut Input<'a>, n: usize) {
    for _ in 0..n {
        wait_for_button(button).await;
    }
}

/// Task waiting for three times an input on `button` and signalling this event through `signal`.
#[embassy_executor::task(pool_size = 2)]
async fn button_task(mut button: Input<'static>, signal: &'static ButtonSignal) {
    loop {
        wait_for_button_n_times(&mut button, 3).await;
        signal.signal(());
    }
}

/// Task performing the board EOL test by orchestrating LED patterns and checking button inputs.
#[embassy_executor::task]
async fn eol_task(
    mut storeys: Storeys<'static>,
    first_button: &'static ButtonSignal,
    second_button: &'static ButtonSignal,
    mut finished_led: LowActiveSwitch<'static>,
) {
    let step = Duration::from_millis(500);

    // Cycle LEDs while waiting for button presses. This should be the most distinguishable action
    // giving the user all the time need for checking the storey LEDs.
    log::info!("start cycling");
    first_button.reset();
    match select(storeys.cycle(step), first_button.wait()).await {
        Either::First(_) => log::info!("cycle done"),
        Either::Second(_) => log::info!("cycle timeout"),
    }

    // Blink all LEDs while waiting for input from the shake sensor.
    log::info!("start blinking");
    second_button.reset();
    match select(storeys.blink(step), second_button.wait()).await {
        Either::First(_) => log::info!("blink done"),
        Either::Second(_) => log::info!("blink timeout"),
    }

    // Done. Light up all storey LEDs and additionally the blue LED on the ESP board.
    storeys.all_on();
    finished_led.switch_on();
}

static SW1_SIGNAL: ButtonSignal = ButtonSignal::new();
static U2_SIGNAL: ButtonSignal = ButtonSignal::new();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer.alarm0);

    // Current flows from the power source (+) directly through the LED, a current limiting
    // resistor, and finally into a GPIO pin with its low-side switching transistor to GND (-).
    //
    // We are using the GPIOs as a low-side switch to the LEDs: no push/pull output, only the
    // low-side transistor of theo output stage as low-side switching transistor for the LED.
    let led_pin_config = OutputConfig::default()
        .with_drive_mode(DriveMode::OpenDrain)
        .with_pull(Pull::None);

    let storey_leds = [
        LowActiveSwitch::new(Output::new(peripherals.GPIO3, Level::High, led_pin_config)),
        LowActiveSwitch::new(Output::new(peripherals.GPIO4, Level::High, led_pin_config)),
        LowActiveSwitch::new(Output::new(peripherals.GPIO21, Level::High, led_pin_config)),
        LowActiveSwitch::new(Output::new(peripherals.GPIO20, Level::High, led_pin_config)),
        LowActiveSwitch::new(Output::new(peripherals.GPIO10, Level::High, led_pin_config)),
        LowActiveSwitch::new(Output::new(peripherals.GPIO7, Level::High, led_pin_config)),
        LowActiveSwitch::new(Output::new(peripherals.GPIO6, Level::High, led_pin_config)),
        LowActiveSwitch::new(Output::new(peripherals.GPIO5, Level::High, led_pin_config)),
    ];
    let storeys = Storeys::new(storey_leds);

    let esp_led = LowActiveSwitch::new(Output::new(peripherals.GPIO8, Level::High, led_pin_config));

    let switch_pin_config = InputConfig::default().with_pull(Pull::Up);
    let sw1 = Input::new(peripherals.GPIO1, switch_pin_config);
    let u2 = Input::new(peripherals.GPIO0, switch_pin_config);

    log::info!("Starting EOL test.");
    // Spawn a debouncing and counting task for each "button". Each triplet of "presses" will
    // generate as signal which is later checked by the EOL task.
    spawner.spawn(button_task(sw1, &SW1_SIGNAL)).unwrap();
    spawner.spawn(button_task(u2, &U2_SIGNAL)).unwrap();
    // Finally spawn the EOL task showing different storey LED patterns for user inspection of LEDs
    // and as a prompt for pressing SW1 or shaking the board for checking the shake sensor U2.
    spawner
        .spawn(eol_task(storeys, &SW1_SIGNAL, &U2_SIGNAL, esp_led))
        .unwrap();

    // Keep the main task running forever.
    loop {
        delay(Duration::from_secs(3)).await;
    }
}
