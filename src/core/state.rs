// src/core/state.rs 

pub struct State {
	// State fields will go here
	pub is_initialized: bool,
}

impl State {
	pub fn default() -> Self{
		Self {
			is_initialized: false,
		}
	}
}