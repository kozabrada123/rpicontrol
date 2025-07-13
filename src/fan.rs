// With the current setup, 4- 5V, 6 and 16 GPIO 23 are used
// GPIO 23 controls the fan; HIGH is on, LOW is off

use rppal;

pub struct FanCtl {
    pub enabled: bool,
    pub controlpin: rppal::gpio::OutputPin,
    _gpiocontroller: rppal::gpio::Gpio,
}

impl FanCtl {
    pub fn new(controlpin: rppal::gpio::OutputPin, gpiocontroller: rppal::gpio::Gpio) -> FanCtl {
        FanCtl {
            enabled: false,
            controlpin: controlpin,
            _gpiocontroller: gpiocontroller,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.controlpin.set_high();
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.controlpin.set_low();
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;

        match self.enabled {
            true => self.controlpin.set_high(),
            false => self.controlpin.set_low(),
        }
    }

    pub fn set(&mut self, enabled: bool) {
        self.enabled = enabled;

        match self.enabled {
            true => self.controlpin.set_high(),
            false => self.controlpin.set_low(),
        }
    }
}

