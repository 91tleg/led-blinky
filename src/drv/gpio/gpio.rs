use crate::GpioMode;
use crate::drv::gpio::gpio_base::GpioBase;
use esp_hal::gpio::{Input, Level, Output};

pub struct Gpio<'a> {
    mode: GpioMode<'a>,
}

impl<'a> Gpio<'a> {
    pub fn new_output(pin: Output<'a>) -> Self {
        Self {
            mode: GpioMode::Output(pin),
        }
    }

    pub fn new_input(pin: Input<'a>) -> Self {
        Self {
            mode: GpioMode::Input(pin),
        }
    }
}

impl<'a> GpioBase for Gpio<'a> {
    fn set_level(&mut self, level: u8) {
        if let GpioMode::Output(ref mut pin) = self.mode {
            pin.set_level(if level != 0 { Level::High } else { Level::Low });
        }
    }

    fn get_level(&self) -> u8 {
        match &self.mode {
            GpioMode::Input(pin) => (pin.level() == Level::High) as u8,
            GpioMode::Output(_) => 0,
        }
    }
}
