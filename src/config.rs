use std::env;
use std::env::VarError;

pub fn get_required_config(value: &str) -> String {
	let result = env::var(value);
	match result {
		Ok(event) => event,
		_ => panic!("Failed to get required env variable, {}", value),
	}
}

pub fn get_config(value: &str) -> Result<String, VarError> {
	env::var(value)
}

pub fn get_watch_directory() -> String {
	get_required_config("WATCH_DIR")
}
pub fn get_dead_letter_directory() -> String {
	get_required_config("DEAD_LETTER_DIR")
}
