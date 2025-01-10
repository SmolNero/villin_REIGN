// src/core/mod.rs
// Core functionality for villin_REIGHN
use std::error;
mod config;	
// 'mod' declares a module
// locate either  config.rs or config/mod.rs
mod state;
// locate either  state.rs or state/mod.rsk
// These are private module declartoins - only visible in scope

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
		
		// Makes public function, to call -> REIGN::new()
	  	// fn new(config: Config) -> defines a func named 'new', takes one argument
		// config: Config -> parameter named 'config' of Type Config
		// -> Result<> specifies return type
		//	   - Result is an enum for success/failure handling		
		//	   - Self refers to the REIGN type
		//	   - Box<dyn std::error::Error> is a trait object for error handling
		
		Ok(Self {
			// Result<T, E> is an enum that represents either:
				// Ok(T) -> Sueccess (returns a valid result type T)
				// Err(E) -> Failure (returns an error of type E)
			// Ok is a success variant of Result
			// Self -> creates new instance of REIGN
			config,
				// Field init shorthand: same as config: config
			state: State::default(),
			   // Calls the default() function on State type
			   // The default() method creates a new instance of State with default values
		})
	}
}

