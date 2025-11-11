#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

mod drv;
use drv::gpio::gpio::Gpio;
use drv::gpio::gpio_base::GpioMode;
use esp_hal::gpio::{Level, Output, OutputConfig};

mod mdl;
use mdl::led::led_task::led_task;

use defmt::info;

// Embassy is an async runtime, Spawner allows spawning async tasks.
use embassy_executor::Spawner;

// Duration and Timer are for delays and sleeping asynchronously (non-blocking).
use embassy_time::{Duration, Timer};

// CpuClock lets configures the CPU frequency.
use esp_hal::clock::CpuClock;

// TimerGroup gives access to hardware timers (used by the RTOS).
use esp_hal::timer::timg::TimerGroup;

//Handles Wi-Fi and Bluetooth functionality.
use esp_radio::ble::controller::BleConnector;

// esp_backtrace provides backtrace and panic support. esp_println gives you printing macros (compatible with no_std).
use {esp_backtrace as _, esp_println as _};

// Enables the use of the alloc crate for dynamic memory (Vec, Box).
extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.0.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744); // Creates a heap in reclaimed RAM
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(size: 64 * 1024);

    // Start RTOS scheduler
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    // BLE WIFI setup
    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");
    let _connector = BleConnector::new(&radio_init, peripherals.BT, Default::default());

    // TODO: Spawn some tasks
    //let _ = spawner;
    let led_pin: Output<'static> =
        Output::new(peripherals.GPIO0, Level::Low, OutputConfig::default());
    let led: Gpio<'static> = Gpio::new_output(led_pin);
    spawner
        .spawn(led_task(led))
        .expect("Failed to spawn LED blink task");

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples/src/bin
}
