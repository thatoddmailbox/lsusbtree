use std::fs;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Device {
    filename: String,
    path: std::path::PathBuf
}

fn main() {
    let devices = fs::read_dir("/sys/bus/usb/devices").unwrap();
    let mut roots: Vec<Device> = Vec::new();

    for entry in devices {
        let device = if let Ok(d) = entry { d } else { continue; };

        let filename_str = device.file_name().into_string();
        let filename: std::string::String = filename_str.unwrap_or("".to_string());
        if !filename.starts_with("usb") {
            continue;
        }

        let path = device.path();

        roots.push(Device {
            filename,
            path
        });
    }

    roots.sort();

    println!("{:?}", roots);
}
