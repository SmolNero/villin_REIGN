// src/core/config.rs

pub struct Config {
	// Configuration fields will go here
	pub api_key: String,
	pub port: u16,
}

impl Config {
	pub fn new(api_key: String, port: u16) -> Self {
		Self {api_key, port}
	}	
}

