extern crate rustpi_io;

use rustpi_io::*;
use rustpi_io::gpio::*;
use rustpi_io::serial::Device;
use std::io::*;

pub enum MCPDeviceType {
	MCP3008,
	MCP3004
}

pub struct MCPDevice {
	dev: serial::SerialPi,
	mcp_type: MCPDeviceType,
	chip_select: GPIO
}

pub struct AnalogIn {
	pin: u8,
	command: u8
}

impl MCPDevice {
	pub fn new(device: Device, mcp_type: MCPDeviceType, chip_select_pin: u8) -> Result<MCPDevice> {
		let mut dev = serial::SerialPi::new(
			device,
			serial::Speed::Khz122,
			serial::SpiMode::Mode0,
			serial::ComMode::FullDuplex)?;
		dev.try_shrink_to(3);
		let chip_select = GPIO::new(chip_select_pin, GPIOMode::Write)?;
		return Ok(MCPDevice { dev, mcp_type, chip_select });
	}

	fn validate_pin(&self, pin: u8) -> Result<()> {
		let max_pin = match &self.mcp_type {
			MCPDeviceType::MCP3004 => 3,
			MCPDeviceType::MCP3008 => 7
		};
		if pin > max_pin {
			Err(Error::new(ErrorKind::InvalidInput, "pin is invalid"))
		} else {
			Ok(())
		}
	}

	fn differential_channel(&self, pin: u8, neg_pin: u8) -> Result<u8> {
		match &self.mcp_type {
			MCPDeviceType::MCP3004 => match (pin, neg_pin) {
				(0, 1) => Ok(0),
				(1, 0) => Ok(1),
				(2, 3) => Ok(2),
				(3, 2) => Ok(3),
				_ => Err(Error::new(ErrorKind::InvalidInput, "invalid pin pair"))
			},
			MCPDeviceType::MCP3008 => match (pin, neg_pin) {
				(0, 1) => Ok(0),
				(1, 0) => Ok(1),
				(2, 3) => Ok(2),
				(3, 2) => Ok(3),
				(4, 5) => Ok(4),
				(5, 4) => Ok(5),
				(6, 7) => Ok(6),
				(7, 6) => Ok(7),
				_ => Err(Error::new(ErrorKind::InvalidInput, "invalid pin pair"))
			}
		}
	}

	pub fn single_analog_in(&self, pin: u8) -> Result<AnalogIn> {
		self.validate_pin(pin)?;
		Ok(AnalogIn { pin, command: 0x03 })
	}

	pub fn differential_analog_in(&self, pin: u8, neg_pin: u8) -> Result<AnalogIn> {
		let pin = self.differential_channel(pin, neg_pin)?;
		Ok(AnalogIn { pin, command: 0x02 })
	}

	pub fn read_value(&mut self, channel: &AnalogIn) -> Result<u16> {
		let command = (channel.command << 6) | (channel.pin << 3);
		let out_buf: [u8; 3] = [command, 0, 0];
		let mut in_buf: [u8; 3] = [0, 0, 0];

		self.chip_select.set(GPIOData::Low)?;
		self.dev.write(&out_buf)?;
		self.dev.read(&mut in_buf)?;
		self.chip_select.set(GPIOData::High)?;

		let value = (((in_buf[0] & 0x01) as u16) << 9)
					| ((in_buf[1] as u16) << 1)
					| (in_buf[2] >> 7) as u16;
		Ok(value)
	}

	pub fn read_voltage(&mut self, channel: &AnalogIn) -> Result<f64> {
		let value = self.read_value(&channel)?;
		Ok((value as f64 * 3.3) / 65535.0)
	}
}