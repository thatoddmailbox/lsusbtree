use std::fs;

fn main() {
    let devices = fs::read_dir("/sys/bus/usb/devices").unwrap();

    for entry in devices {
        let device = if let Ok(d) = entry { d } else { continue; };
        println!("{:?}", device.file_name());
    }
}
