// src/core/config.rs

pub struct Config {
	// Configuration fields will go here
	pub api_key: String,
	pub Port: u16,
}

impl Config {
	pub fn new(api_key: String, Port: u16) -> Self {
		Self {api_key, port}
	}	
}

