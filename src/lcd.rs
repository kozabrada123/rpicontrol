// HEHEY FUCKO! This needs 5V and you only have 3.3V. THATS WHY IT DIDNT WORK

// Current wirings of lcd
// PIN - Connection (Rpi pin, wire)

// VSS - GND (6, Brown 1) Minus
// VDD - 5v (2, Red 1)  Plus
// VO - PWM0, GPIO 12 (32, Orange) Kontrast
// RS - GPIO 23 (16, Yellow) Register Select
// RW - GND (14, Green) Read Write, vedno minus ker samo writamo
// E - GPIO 24 (18, Blue) Za Branje, for some reason gpio
// D0 - D3 - None
// D4 - GPIO 4 (7, Black)
// D5 - GPIO 17 (11, White)
// D6 - GPIO 27 (13, Gray)
// D7 - GPIO 22 (15, Purple)
// A - PWM1, GPIO 13 (33, Red 2) Svetlost backlighta
// K - GND (34, Brown 2) Backlight GND

use core::fmt::Write; // for write!
use lcd::*;
use std::{thread, time};
use rppal;

// implement HAL...
pub struct HW {
    // any data needed to access low-level 
    Gpio: rppal::gpio::Gpio,
    PinVo: rppal::pwm::Pwm, //PWM0
    PinRs: rppal::gpio::OutputPin,
    PinE: rppal::gpio::OutputPin,
    PinD4: rppal::gpio::OutputPin,
    PinD5: rppal::gpio::OutputPin,
    PinD6: rppal::gpio::OutputPin,
    PinD7: rppal::gpio::OutputPin,
    PinA: rppal::pwm::Pwm, //PWM1
}

impl HW {
    // Make a new HW with the pin nums
    pub fn new(
        vo: u8,
        rs: u8,
        e: u8,
        d4: u8,
        d5: u8,
        d6: u8,
        d7: u8,
        a: u8,
        brightness: f64,
    ) -> Self {
        let mut gpio = rppal::gpio::Gpio::new().unwrap();

        let mut out = Self {
            // Turn the u8s to pins
            Gpio: gpio.clone(),
            PinVo: rppal::pwm::Pwm::with_frequency(rppal::pwm::Channel::Pwm0, 490.0, 0.0, rppal::pwm::Polarity::Normal, true).unwrap(),
            PinRs: gpio.get(rs).unwrap().into_output(),
            PinE: gpio.get(e).unwrap().into_output(),
            PinD4: gpio.get(d4).unwrap().into_output(),
            PinD5: gpio.get(d5).unwrap().into_output(),
            PinD6: gpio.get(d6).unwrap().into_output(),
            PinD7: gpio.get(d7).unwrap().into_output(),
            PinA: rppal::pwm::Pwm::with_frequency(rppal::pwm::Channel::Pwm1, 490.0, brightness, rppal::pwm::Polarity::Normal, true).unwrap(),
        };

        out.PinE.set_low();
        out.PinD4.set_low();
        out.PinD5.set_low();
        out.PinD6.set_low();
        out.PinD7.set_low();

        out.PinVo.enable();
        out.PinA.enable();

        out
    }

    // Configured new from the thing in the top of the file
    pub fn cur() -> Self {
        HW::new(12, 23, 24, 4, 17, 27, 22, 13, 0.9)
    }
}

// implement `Hardware` trait to give access to LCD pins
impl Hardware for HW {
    fn rs(&mut self, bit: bool) {
        // should set R/S pin on LCD screen
        match bit {
            true => self.PinRs.set_high(),
            false => self.PinRs.set_low(),
        }
    }
    fn enable(&mut self, bit: bool) {
        // should set EN pin on LCD screen
        // UHHHHH I hope this is E?
        match bit {
            true => self.PinE.set_high(),
            false => self.PinE.set_low(),
        }

    }
    fn data(&mut self, data: u8) {
        // should set data bits to the LCD screen (only lowest 4 bits are used in 4-bit mode).

        let length = data.count_ones() + data.count_zeros();
        for n in 0..length {
            println!("{}", data.clone() >> n & 1);
        }

        println!("{}", data.clone());

        println!("Writing to D4 {}", data.clone() >> 0 & 1);
        println!("Writing to D5 {}", data.clone() >> 1 & 1);
        println!("Writing to D6 {}", data.clone() >> 2 & 1);
        println!("Writing to D7 {}", data.clone() >> 3 & 1);

        // Dumb as
        match data >> 0 & 1 {
            1 => self.PinD4.set_high(),
            0 => self.PinD4.set_low(),
            _ => panic!("A boolean is not a boolean?????? Your computer may have been hit by a cosmic ray??"),
        }

        match data >> 1 & 1 {
            1 => self.PinD5.set_high(),
            0 => self.PinD5.set_low(),
            _ => panic!("A boolean is not a boolean?????? Your computer may have been hit by a cosmic ray??"),

        }

        match data >> 2 & 1 {
            1 => self.PinD6.set_high(),
            0 => self.PinD6.set_low(),
            _ => panic!("A boolean is not a boolean?????? Your computer may have been hit by a cosmic ray??"),

        }

        match data >> 3 & 1 {
            1 => self.PinD7.set_high(),
            0 => self.PinD7.set_low(),
            _ => panic!("A boolean is not a boolean?????? Your computer may have been hit by a cosmic ray??"),

        }

        /* This was actually nice :cope:
        self.PinD4.set_value(data >> 0 & 1).unwrap();
        self.PinD5.set_value(data >> 1 & 1).unwrap();
        self.PinD6.set_value(data >> 2 & 1).unwrap();
        self.PinD7.set_value(data >> 3 & 1).unwrap();*/
    }

    // optionally, override the following function to switch to 8-bit mode
    fn mode(&self) -> lcd::FunctionMode {
        lcd::FunctionMode::Bit4
    }

    // optionally, implement the following three functions to enable polling busy flag instead of delay
    fn can_read(&self) -> bool {
        false
    }

    fn rw(&mut self, bit: bool) {
        // configure pins for input _before_ setting R/W to 1
        // configure pins for output _after_ setting R/W to 0
    }
    fn read_data(&mut self) -> u8 {
        0 // read data from the port
        // Nop
    }
}

// implement `Delay` trait to allow library to sleep for the given amount of time
impl Delay for HW {
    fn delay_us(&mut self, delay_usec: u32) {
        // should sleep for the given amount of microseconds
        let in_us = time::Duration::from_micros(delay_usec.into());
        thread::sleep(in_us);
    }
}

// Function that makes an lcd
pub fn init() -> Display<HW> {
    // create HAL and LCD instances
    let hw = HW::cur();
    let mut lcd = Display::new(hw);

    // initialization
    lcd.init(FunctionLine::Line2, FunctionDots::Dots5x8);
    lcd.display(
        DisplayMode::DisplayOn,
        DisplayCursor::CursorOff,
        DisplayBlink::BlinkOff,
    );
    lcd.entry_mode(EntryModeDirection::EntryRight, EntryModeShift::NoShift);

    return lcd;
}
