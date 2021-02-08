use std::fs;

fn main() {
    let devices = fs::read_dir("/sys/bus/usb/devices").unwrap();

    for device in devices {
        println!("{}", device.unwrap().path().display())
    }
}
