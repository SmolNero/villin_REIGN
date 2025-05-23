// src/core/config.rs

pub struct Config {
	pub api_key: String,
	pub port: u16,				
	pub security_level: u8,	  
}

impl Config {
	pub fn new(api_key: String, port: u16, security_level:u8) -> Result<Self, &'static str> {
		if !(1..=5).contains(&security_level) {
			return Err("Security level must be between 1 and 5");
		}
		
		if api_key.trim().is_empty(){ 	// Validate API key is not empty
			return Err("API key cannot be empty");
		}

		Ok(Self {
			api_key,
			port,
			security_level,
		})
	}

	pub fn default() -> Self {
		Self {
			api_key: String::from("default_key"),
			port: 8080,
			security_level: 1,
		}	
	}
}	


