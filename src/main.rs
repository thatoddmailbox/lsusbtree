use std::fs;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Root {
    filename: String,
    path: std::path::PathBuf
}

fn main() {
    let devices = fs::read_dir("/sys/bus/usb/devices").unwrap();
    let mut roots: Vec<Root> = Vec::new();

    for entry in devices {
        let device = if let Ok(d) = entry { d } else { continue; };

        let filename_str = device.file_name().into_string();
        let filename: std::string::String = filename_str.unwrap_or("".to_string());
        if !filename.starts_with("usb") {
            continue;
        }

        let path = device.path();

        roots.push(Root {
            filename,
            path
        });
    }

    roots.sort();

    println!("{:?}", roots);
}
