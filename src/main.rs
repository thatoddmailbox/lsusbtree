use std::fs;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Device {
    filename: String,
    path: std::path::PathBuf
}

fn read_device_file(d: &Device, filename: &str) -> String {
    return std::fs::read_to_string(d.path.join(filename)).unwrap_or("".to_string()).trim().to_string();
}

fn print_info(d: &Device) {
    let vid = read_device_file(d, "idVendor");
    let pid = read_device_file(d, "idProduct");
    let manufacturer = read_device_file(d, "manufacturer");
    let product = read_device_file(d, "product");
    println!("{} {} {} {}", vid, pid, manufacturer, product);
}

fn descend(d: Device) {
    println!("{:?}", d);
    print_info(&d);

    let children = fs::read_dir(d.path).unwrap();
    for entry in children {
        let child = if let Ok(d) = entry { d } else { continue; };

        let filename_str = child.file_name().into_string();
        let filename = filename_str.unwrap_or("".to_string());
        if !filename.chars().next().unwrap().is_numeric() {
            continue;
        }

        println!("{:?}", child);
    }
}

fn main() {
    let devices = fs::read_dir("/sys/bus/usb/devices").unwrap();
    let mut roots: Vec<Device> = Vec::new();

    for entry in devices {
        let device = if let Ok(d) = entry { d } else { continue; };

        let filename_str = device.file_name().into_string();
        let filename = filename_str.unwrap_or("".to_string());
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

    for root in roots {
        descend(root);
    }
}
