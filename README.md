# lsusbtree
A tool that prints out your USB devices (including manufacturer and product info) in a tree format. Think of it like `lsusb -t`, but useful.

It reads your USB devices from `/sys/bus/usb/devices`, so it currently only supports Linux.

## Sample output
![Output](./output.png)

## Troubleshooting
As mentioned, this tool reads all of its data directly from `/sys/bus/usb/devices`, so if you think the data is wrong, you should check Linux's logs. `dmesg` is normally pretty helpful, and will tell you if there are errors preventing a device from enumerating.

If the tool seems to freeze halfway through displaying the tree, this usually indicates that Linux has detected a device, but for whatever reason is unable to read its descriptors. `dmesg` can also help here, since you will be able to see the errors, if any.

## TODO
* Give the option to read from `usb.ids` instead of the manufacturer/product descriptors
* Display speed (low, high, full, super, super+) for each device (or, if that's not possible, at least the version: 1.0, 2.0, 3.0, etc)
* Handle blank manufacturer/product descriptors better