use esp_hal::gpio::{Input, Output};

pub enum GpioMode<'a> {
    Input(Input<'a>),
    Output(Output<'a>),
}

pub trait GpioBase {
    /// Set the pin level (0 = low, 1 = high)
    fn set_level(&mut self, level: u8);

    /// Get the pin level
    fn get_level(&self) -> u8;
}
