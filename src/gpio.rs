use std::fs::OpenOptions;
use std::os::unix::prelude::*;
use {get_chipinfo, gpiochip_info};

pub struct Gpio {
    fd: RawFd,
}

impl Gpio {
    pub fn new(path: &str) -> Self {
        Gpio {
            fd: OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .expect(format!("Could not open {} for reading and writing.", path).as_str())
                .as_raw_fd(),
        }
    }

    pub fn info(&self) {
        let ref mut info = gpiochip_info {
            name: [0; 32],
            label: [0; 32],
            lines: 0,
        };

        let status = unsafe { get_chipinfo(self.fd, info) };

        println!("status: {}", status);
        println!("name: {:?}", info.name);
        println!("label: {:?}", info.label);
        println!("lines: {:?}", info.lines);
    }
}
