use std::fs;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Device {
    filename: String,
    path: std::path::PathBuf
}

fn read_device_file(d: &Device, filename: &str) -> String {
    return std::fs::read_to_string(d.path.join(filename)).unwrap_or("".to_string()).trim().to_string();
}

fn print_info(d: &Device, level: usize, have_children: bool) {
    let vid = read_device_file(d, "idVendor");
    let pid = read_device_file(d, "idProduct");
    let manufacturer = read_device_file(d, "manufacturer");
    let product = read_device_file(d, "product");

    let offset = if level > 0 {
        "  ".repeat(level - 1) + "└─"
    } else { "".to_owned() };

    let indicator = if have_children { "┬" } else { "─" };

    println!("{}{} {} {} {} {} ({})", offset, indicator, vid, pid, manufacturer, product, d.filename);
}

fn descend(d: Device, level: usize) {
    let children = fs::read_dir(d.path.as_path()).unwrap();
    let mut children_devices: Vec<Device> = Vec::new();

    for entry in children {
        let child = if let Ok(d) = entry { d } else { continue; };

        let filename_str = child.file_name().into_string();
        let filename = filename_str.unwrap_or("".to_string());
        if !filename.chars().next().unwrap().is_numeric() {
            continue;
        }

        let path = child.path();

        let interface_file = path.join("bInterfaceClass");
        if interface_file.exists() {
            // it's actually an interface, ignore it
            continue;
        }

        children_devices.push(Device {
            filename,
            path
        });
    }

    children_devices.sort();

    print_info(&d, level, children_devices.len() > 0);

    for child_device in children_devices {
        descend(child_device, level + 1);
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

    let mut first = true;
    for root in roots {
        if first {
            first = false;
        } else {
            println!("");
        }
        descend(root, 0);
    }
}
