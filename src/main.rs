extern crate spin_sleep;
extern crate wiringpi;

mod display;
use display::Display;

fn main() {
    let gpio = wiringpi::setup_gpio();
    let display = Display::new(&gpio);

    loop {
        display.print(['f', 'u', 'c', 'k']);
    }
}
