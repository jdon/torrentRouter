extern crate notify;

mod config;
use lava_torrent::torrent::v1::Torrent;
use notify::{DebouncedEvent, PollWatcher, RecursiveMode, Watcher};
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

fn copy_file(file_path: &std::path::PathBuf, new_file_path: &std::path::PathBuf) {
	match fs::copy(&file_path, &new_file_path) {
		Ok(bytes_written) => {
			println!(
				"Copied file: {:?} to {:?}. {} bytes written",
				&file_path, &new_file_path, &bytes_written
			);
			delete_file(&file_path);
		}
		Err(error) => {
			handle_failure(
				&file_path,
				format!(
					"Failed to copy file to {:?}. Error {}",
					&new_file_path, error
				),
			);
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

	// Add a path to be watched. All files and directories at that path and
	// below will be monitored for changes.
	watcher.watch(watch_directory, RecursiveMode::NonRecursive)?;

	// This is a simple loop, but you may want to use more complex logic here,
	// for example to handle I/O.
	loop {
		match rx.recv() {
			Ok(event) => {
				if let DebouncedEvent::Create(file_path) = event {
					if let Some(extension) = file_path.extension() {
						if extension == "torrent" {
							if let Some(file_name) = file_path.file_name() {
								if let Ok(torrent) = Torrent::read_from_file(&file_path) {
									if let Some(announce) = torrent.announce {
										if let Ok(url) = Url::parse(&announce) {
											if let Some(domain) = url.domain() {
												let env_value =
													domain.to_uppercase().replace('.', "_");
												if let Ok(folder_dir) =
													config::get_config(&env_value)
												{
													let tracker_file_path =
														Path::new(&folder_dir).join(&file_name);
													copy_file(&file_path, &tracker_file_path);
												} else {
													handle_failure(
														&file_path,
														format!(
														"Failed to get environment variable: {:?}",
														env_value
													),
													);
													let dead_letter_file_path =
														Path::new(&dead_letter_directory)
															.join(&file_name);
													copy_file(&file_path, &dead_letter_file_path);
												}
											} else {
												handle_failure(
													&file_path,
													format!(
														"Failed to parse domain from URL: {:?}",
														url
													),
												);
											}
										} else {
											handle_failure(
												&file_path,
												format!(
													"Failed to parse announce URL: {:?}",
													announce
												),
											);
										}
									} else {
										handle_failure(
											&file_path,
											format!(
												"Failed read announce of torrent file: {:?}",
												file_name
											),
										);
									}
								} else {
									handle_failure(
										&file_path,
										format!("Failed to read torrent file: {:?}", file_name),
									);
								}
							}
						}
					}
				}
			}
			Err(e) => println!("Watch error: {:?}", e),
		}
	}
}

fn main() {
	if let Err(e) = watch() {
		println!("error: {:?}", e)
	}
}
