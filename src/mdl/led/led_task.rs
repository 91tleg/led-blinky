use defmt::info;
use embassy_time::{Duration, Timer};
use crate::drv::gpio::gpio::Gpio;
use crate::drv::gpio::gpio_base::GpioBase;

fn led_toggle(level: u8) -> u8 {
    level ^ 1
}
/// Asynchronous LED blink task.
///
/// This continuously toggles the LED using your Gpio wrapper.
#[embassy_executor::task]
pub async fn led_task(mut led: Gpio<'static>) {
    let mut level = 0u8;

    loop {
        info!("LED state = {}", level);
        led.set_level(level);
        led_toggle(level);
        Timer::after(Duration::from_secs(1)).await;
    }
}
