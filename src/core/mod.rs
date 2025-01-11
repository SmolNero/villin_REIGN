// src/core/mod.rs
// Core functionality for villin_REIGHN

use std::error;
mod config;	
// 'mod' declares a module
// locate either  config.rs or config/mod.rs
mod state;
// locate either  state.rs or state/mod.rsk
// These are private module declartoins - only visible in scope
mod display;


// re-export statement that consists of:
pub use config::Config; 	// public -> 'use' type in scope -> This allows other code to use Config directly
pub use state::State;		// state::State - from state module,import State type

pub struct REIGN {			// Public struct -> 'struct' defines a new structure
	config: Config,				//Field named 'config' of a type Config
	state: State,				// 'state' -> type State
}

impl REIGN {		
		// Adding methdods to REIGN struct
		// All functions (methods) -> to REIGN stuct above		
	pub fn new(config: Config) -> Result<Self, Box<dyn error::Error>> {
		let reign = Self {
			config,
			state: State::default(),
		};
		println!("Initializing REIGN with:",);
		println!("___________________________________");
		println!("|	    Port 	|  {} |", reign.config.port);
		println!("|	Security Level  |   {}	|", reign.config.security_level);
		println!("___________________________________");
		//initialize state
		reign.init_state()
	}

	fn init_state(&self) -> Result<Self, Box<dyn std::error::Error>> {
		if !self.state.is_initialized {
			println!("System state initialized  ⭕	");
		}
		Ok(Self {
			config: Config {
				api_key: self.config.api_key.clone(),
				port: self.config.port,
				security_level: self.config.security_level,
			},
			state: State::default(),
		})
	}

	pub fn security_level(&self) -> u8 {
		self.config.security_level
	}
}

