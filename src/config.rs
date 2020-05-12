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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic]
	fn should_panic_on_invalid_config() {
		env::remove_var("VALID_ENV_VAR");
		get_required_config("INVALID_ENV_VAR");
	}

	#[test]
	#[should_panic]
	fn should_panic_on_watch_directory() {
		env::remove_var("WATCH_DIR");
		get_watch_directory();
	}

	#[test]
	#[should_panic]
	fn should_panic_on_dead_letter_directory() {
		env::remove_var("DEAD_LETTER_DIR");
		get_dead_letter_directory();
	}
	#[test]
	fn should_get_watch_directory() {
		env::set_var("WATCH_DIR", "ENV_VALUE");
		assert_eq!(get_watch_directory(), "ENV_VALUE");
	}
	#[test]
	fn should_get_dead_letter_directory() {
		env::set_var("DEAD_LETTER_DIR", "ENV_VALUE");
		assert_eq!(get_dead_letter_directory(), "ENV_VALUE");
	}
	#[test]
	fn should_get_valid_required_config() {
		env::set_var("VALID_ENV_VAR", "ENV_VALUE");
		assert_eq!(get_required_config("VALID_ENV_VAR"), "ENV_VALUE");
	}
}
