use spin_sleep::SpinSleeper;
use std::iter::Enumerate;
use std::slice::Iter;
use wiringpi::WiringPi;
use wiringpi::pin::Gpio as WiringPiGpio;
use wiringpi::pin::OutputPin as WiringPiOutputPin;
use wiringpi::pin::Value;
use wiringpi::pin::Value::{High, Low};

type Gpio = WiringPi<WiringPiGpio>;
type OutputPin = WiringPiOutputPin<WiringPiGpio>;
type PinNum = u16;

const LETTERS: [[Value; 8]; 25] = [
    [High, High, High, High, High, High, Low, Low],
    [Low, High, High, Low, Low, Low, Low, Low],
    [High, High, Low, High, High, Low, High, Low],
    [High, High, High, High, Low, Low, High, Low],
    [Low, High, High, Low, Low, High, High, Low],
    [High, Low, High, High, Low, High, High, Low],
    [High, Low, High, High, High, High, High, Low],
    [High, High, High, Low, Low, Low, Low, Low],
    [High, High, High, High, High, High, High, Low],
    [High, High, High, High, Low, High, High, Low],
    [Low, Low, Low, Low, Low, Low, Low, High],
    [High, High, High, Low, High, High, High, Low],
    [High, High, High, High, High, High, High, Low],
    [High, Low, Low, High, High, High, Low, Low],
    [Low, Low, High, High, High, High, High, Low],
    [High, Low, Low, High, High, High, High, Low],
    [High, Low, Low, Low, High, High, High, Low],
    [Low, High, High, Low, High, High, High, Low],
    [Low, High, High, Low, High, High, High, Low],
    [Low, Low, Low, High, High, High, Low, Low],
    [High, High, High, High, High, High, Low, Low],
    [High, High, High, Low, High, High, High, Low],
    [High, Low, High, High, Low, High, High, Low],
    [Low, High, High, High, High, High, Low, Low],
    [Low, Low, Low, Low, Low, Low, High, Low],
];

fn pin(gpio: &Gpio, pin: PinNum) -> OutputPin {
    let pin = gpio.output_pin(pin);
    pin.digital_write(Low);
    pin
}

pub struct Display {
    digit_pins: [OutputPin; 4],
    segment_pins: [OutputPin; 8],
    sleeper: SpinSleeper,
}

impl Display {
    pub fn new(gpio: &Gpio) -> Self {
        Display {
            digit_pins: [pin(gpio, 22), pin(gpio, 27), pin(gpio, 17), pin(gpio, 24)],
            segment_pins: [
                pin(gpio, 11),
                pin(gpio, 4),
                pin(gpio, 23),
                pin(gpio, 8),
                pin(gpio, 7),
                pin(gpio, 10),
                pin(gpio, 18),
                pin(gpio, 25),
            ],
            sleeper: SpinSleeper::default(),
        }
    }

    pub fn print(&self, chars: [char; 4]) {
        for (i, digit_pin) in self.digit_pins.iter().enumerate() {
            let states = LETTERS[match chars[i] {
                                     '0' => 0,
                                     '1' => 1,
                                     '2' => 2,
                                     '3' => 3,
                                     '4' => 4,
                                     '5' => 5,
                                     '6' => 6,
                                     '7' => 7,
                                     '8' => 8,
                                     '9' => 9,
                                     '.' => 10,
                                     'a' => 11,
                                     'b' => 12,
                                     'c' => 13,
                                     'd' => 14,
                                     'e' => 15,
                                     'f' => 16,
                                     'h' => 17,
                                     'k' => 18,
                                     'l' => 19,
                                     'o' => 20,
                                     'r' => 21,
                                     's' => 22,
                                     'u' => 23,
                                     _ => 24,
                                 }];

            for (j, segment_pin) in self.segment_pins.iter().enumerate() {
                segment_pin.digital_write(states[j]);
            }

            digit_pin.digital_write(Low);
            self.sleeper.sleep_ns(250);
            digit_pin.digital_write(High);
        }
    }
}
