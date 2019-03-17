extern crate mcp3xxx;

use std::thread;
use std::time::Duration;
use mcp3xxx::*;

fn main() {
	let mut mcp = MCPDevice::new(rustpi_io::serial::Device::CE0, MCPDeviceType::MCP3008, 23)
		.expect("can't open device");

	let ch0 = mcp.single_analog_in(0)
		.expect("can't get analog in channel");

	loop {
		println!("{}", mcp.read_value(&ch0).expect("can't read value"));
		thread::sleep(Duration::from_millis(500));
	}
}