use crate::drv::gpio::gpio::Gpio;
use crate::drv::gpio::gpio_base::GpioBase;
use defmt::info;
use embassy_time::{Duration, Timer};

fn led_toggle(level: u8) -> u8 {
    level ^ 1
}

#[embassy_executor::task]
pub async fn led_task(mut led: Gpio<'static>) {
    let level = 0u8;

    loop {
        info!("LED state = {}", level);
        led.set_level(level);
        led_toggle(level);
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_led_toggle() {
        let level = 0u8;
        let result = 0u8;
        result = led_toggle(level);
        assert_eq!(result, 1);
    }
}
