// Core functionality for villin_REIGHN

mod config;	
// 'mod' declares a module
// locate either  config.rs or config/mod.rs
mod state;
// locate either  state.rs or state/mod.rs
// These are private module declartoins - only visible in scope

// re-export statement that consists of:
pub use config::Config; 	// public -> 'use' type in scope -> This allows other code to use Config directly
pub use state::State;		// state::State - from state module,import State type

pub struct REIGN {			// Public struct -> 'struct' defines a new structure
	config: Config,				//Field
	state: State,
}

impl REIGN {
	pub fn new(config: Config) -> Result<Self, Box<dyn::error::Error>> {
		Ok(Self {
			config,
			state: State::default(),
		})
	}
}
