// Core functionality for villin_REIGHN
mod config;
mod state;

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
