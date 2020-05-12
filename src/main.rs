extern crate notify;

mod config;
use lava_torrent::torrent::v1::Torrent;
use notify::{DebouncedEvent, PollWatcher, RecursiveMode, Watcher};

use std::error;
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use url::Url;

fn handle_failure(file_path: &std::path::PathBuf, error_message: String) {
	eprintln!("{:?}: {}", file_path, error_message);
}

fn delete_file(file_path: &std::path::PathBuf) {
	if let Err(e) = fs::remove_file(file_path) {
		handle_failure(file_path, format!("Failed to remove file: Error: {}", e))
	}
}

fn copy_file(
	file_path: &std::path::PathBuf,
	new_file_path: &std::path::PathBuf,
) -> Result<(), std::io::Error> {
	let bytes_written = fs::copy(&file_path, &new_file_path)?;
	println!(
		"Copied file: {:?} to {:?}. {} bytes written",
		&file_path, &new_file_path, &bytes_written
	);
	delete_file(&file_path);
	Ok(())
}

fn get_file_name(file_path: &std::path::PathBuf) -> Option<&std::ffi::OsStr> {
	let extension = file_path.extension()?;
	if extension != "torrent" {
		return None;
	}
	let file_name = file_path.file_name()?;
	Some(file_name)
}

fn handle_create_event(
	file_path: &std::path::PathBuf,
	file_name: &std::ffi::OsStr,
	dead_letter_file_path: &std::path::PathBuf,
) -> Result<(), Box<dyn error::Error>> {
	let torrent = Torrent::read_from_file(&file_path)?;
	let announce = torrent
		.announce
		.ok_or_else(|| "Failed to read announce URL")?;
	let url = Url::parse(&announce)?;
	let domain = url
		.domain()
		.ok_or_else(|| "Failed to get domain from announce URL")?;
	let env_value = domain.to_uppercase().replace('.', "_");
	let folder_dir = config::get_config(&env_value)?;
	let tracker_file_path = Path::new(&folder_dir).join(&file_name);
	match copy_file(&file_path, &tracker_file_path) {
		Ok(()) => Ok(()),
		Err(_) => {
			copy_file(&file_path, &dead_letter_file_path)?;
			Ok(())
		}
	}
}

fn watch() -> notify::Result<()> {
	let watch_directory = config::get_watch_directory();
	let dead_letter_directory = config::get_dead_letter_directory();

	// Create a channel to receive the events.
	let (tx, rx) = channel();

	// Automatically select the best implementation for your platform.
	// You can also access each implementation directly e.g. INotifyWatcher.
	let mut watcher: PollWatcher = PollWatcher::new(tx, Duration::from_secs(2))?;
	println!("Watching: {}", &watch_directory);
	// Add a path to be watched. All files and directories at that path and
	// below will be monitored for changes.
	watcher.watch(watch_directory, RecursiveMode::NonRecursive)?;

	// This is a simple loop, but you may want to use more complex logic here,
	// for example to handle I/O.
	loop {
		match rx.recv() {
			Ok(event) => {
				if let DebouncedEvent::Create(file_path) = event {
					match get_file_name(&file_path) {
						Some(file_name) => {
							let dead_letter_file_path =
								Path::new(&dead_letter_directory).join(&file_name);
							match handle_create_event(
								&file_path,
								&file_name,
								&dead_letter_file_path,
							) {
								Ok(()) => println!("Successfully processed file: {:?}", &file_path),
								Err(e) => {
									handle_failure(
										&file_path,
										format!("Failed to handle file. Error: {:?}", e),
									);
									copy_file(&file_path, &dead_letter_file_path);
								}
							}
						}
						None => {
							handle_failure(&file_path, "Is not a torrent".to_owned());
						}
					}
				}
			}
			Err(e) => println!("Channel error: {:?}", e),
		}
	}
}

fn main() {
	if let Err(e) = watch() {
		println!("error: {:?}", e)
	}
}
