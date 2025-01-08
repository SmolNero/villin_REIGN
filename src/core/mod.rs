// Core functionality for villin_REIGHN

mod config;	
// 'mod' declares a module
// locate either  config.rs or config/mod.rs
mod state;
// locate either  state.rs or state/mod.rs
// These are private module declartoins


// re-export statement that consists of:
pub use config::Config;
pub use state::State;

pub struct REIGN {
	config: Config,
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
