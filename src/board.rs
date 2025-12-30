//! Board support for the Hakkaa board.

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{DriveMode, Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::timer::systimer::SystemTimer;

use crate::led::STOREY_LEDS;
use crate::switch::LowActiveSwitch;

use esp_hal::interrupt::software::SoftwareInterruptControl;

/// Hakkaa board resources.
pub struct Board<'a> {
    /// The outputs for driving the storey LEDs _D1_ to _D8_ on the main board.
    pub storey_leds: [LowActiveSwitch<'a>; STOREY_LEDS],
    /// The output for driving the blue LED on the ESP32-C3 board _U1_.
    pub esp_led: LowActiveSwitch<'a>,
    /// The input the push putton _SW1_ on the main board is connected to.
    pub sw1: Input<'a>,
    /// The input the shake sensor _U2_ on the main board is connected to.
    pub u2: Input<'a>,
}

impl<'a> Board<'a> {
    /// Initialize the board and returns all the resources *once*.
    ///
    /// # Panics
    ///
    /// Initializing the board and returning its resources is designed to be performed only once.
    /// Subsequent calls to this function will cause a panic from the underlying initializations.
    pub fn init() -> Self {
        esp_println::logger::init_logger_from_env();

        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);

        esp_alloc::heap_allocator!(size: 64 * 1024);

        let timer = SystemTimer::new(peripherals.SYSTIMER);
        let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
        esp_rtos::start(timer.alarm0, sw_int.software_interrupt0);

        // Storey LEDs and the LED on the ESP32-C3 board are connected as follows: Current flows
        // from the power source (+) directly through the LED, a current limiting resistor, and
        // finally into a GPIO pin with its low-side switching transistor to GND (-).
        //
        // We are using the GPIOs as a low-side switch to the LEDs: no push/pull output, only the
        // low-side transistor of theo output stage as low-side switching transistor for the LED.
        let led_pin_config = OutputConfig::default()
            .with_drive_mode(DriveMode::OpenDrain)
            .with_pull(Pull::None);
        let led_pin_init_level = Level::High;

        let storey_leds = [
            LowActiveSwitch::new(Output::new(
                peripherals.GPIO3,
                led_pin_init_level,
                led_pin_config,
            )),
            LowActiveSwitch::new(Output::new(
                peripherals.GPIO4,
                led_pin_init_level,
                led_pin_config,
            )),
            LowActiveSwitch::new(Output::new(
                peripherals.GPIO21,
                led_pin_init_level,
                led_pin_config,
            )),
            LowActiveSwitch::new(Output::new(
                peripherals.GPIO20,
                led_pin_init_level,
                led_pin_config,
            )),
            LowActiveSwitch::new(Output::new(
                peripherals.GPIO10,
                led_pin_init_level,
                led_pin_config,
            )),
            LowActiveSwitch::new(Output::new(
                peripherals.GPIO7,
                led_pin_init_level,
                led_pin_config,
            )),
            LowActiveSwitch::new(Output::new(
                peripherals.GPIO6,
                led_pin_init_level,
                led_pin_config,
            )),
            LowActiveSwitch::new(Output::new(
                peripherals.GPIO5,
                led_pin_init_level,
                led_pin_config,
            )),
        ];
        let esp_led = LowActiveSwitch::new(Output::new(
            peripherals.GPIO8,
            led_pin_init_level,
            led_pin_config,
        ));

        let switch_pin_config = InputConfig::default().with_pull(Pull::Up);
        let sw1 = Input::new(peripherals.GPIO1, switch_pin_config);
        let u2 = Input::new(peripherals.GPIO0, switch_pin_config);

        // TODO: Re-expose the remaining peripherals (through a board-specific) peripherals struct.

        Board {
            storey_leds,
            esp_led,
            sw1,
            u2,
        }
    }
}
