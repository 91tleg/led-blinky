use crate::drv::gpio::gpio_base::GpioBase;
use esp_hal::gpio::Output;

/// Simple GPIO wrapper using esp-hal only
pub struct Gpio<'a> {
    pin: Output<'a>,
    state: u8,
}

impl<'a> Gpio<'a> {
    pub fn new(pin: Output<'a>) -> Self {
        Self { pin, state: 0 }
    }
}

impl<'a> GpioBase for Gpio<'a> {
    fn set_level(&mut self, level: u8) {
        self.state = level;
        if level != 0 {
            self.pin.set_high();
        } else {
            self.pin.set_low();
        }
    }

    fn get_level(&self) -> u8 {
        self.state
    }
}
