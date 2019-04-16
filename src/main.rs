extern crate mcp3xxx;

use std::thread;
use std::time::Duration;
use mcp3xxx::*;

fn main() {
	let mut mcp = MCPDevice::new(rustpi_io::serial::Device::CE0, MCPDeviceType::MCP3008, 22)
		.expect("can't open device");

	let ch0 = mcp.single_analog_in(0)
		.expect("can't get analog in channel 0");

	let ch1 = mcp.single_analog_in(1)
		.expect("can't get analog in channel 1");

	let ch2 = mcp.single_analog_in(2)
		.expect("can't get analog in channel 2");

	loop {
		let r0 = mcp.read_value(&ch0).expect("can't read value from channel 0");
		let r1 = mcp.read_value(&ch1).expect("can't read value from channel 1");
		let r2 = mcp.read_value(&ch2).expect("can't read value from channel 2");
		println!("{} {} {}", r0, r1, r2);
		thread::sleep(Duration::from_millis(500));
	}
}