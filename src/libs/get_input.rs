use input::{
	event::{
		keyboard::{
			KeyState,
			KeyboardEventTrait,
		},
		KeyboardEvent::Key,
	},
	Event,
	Libinput,
	LibinputInterface,
};
use libc::{
	O_RDONLY,
	O_RDWR,
	O_WRONLY,
};
use std::{
	fs::{
		File,
		OpenOptions,
	},
	os::unix::{
		fs::OpenOptionsExt,
		io::OwnedFd,
	},
	path::Path,
};

pub struct Interface;

impl LibinputInterface for Interface {
	fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
		OpenOptions::new()
			.custom_flags(flags)
			.read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
			.write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
			.open(path)
			.map(|file| file.into())
			.map_err(|err| err.raw_os_error().unwrap())
	}
	fn close_restricted(&mut self, fd: OwnedFd) {
		drop(File::from(fd));
	}
}

pub fn new() -> Libinput {
	let mut input = Libinput::new_with_udev(Interface);
	input.udev_assign_seat("seat0").unwrap();

	return input;
}

pub fn detect_keypress(mut input: Libinput, mut keys: Vec<String>) -> Option<Vec<String>> {
	input.dispatch().unwrap();

	let key_events = input.clone().into_iter().filter_map(|event| {
		if let Event::Keyboard(Key(event)) = event {
			Some((event.key_state(), event.key().to_string()))
		} else {
			None
		}
	});

	let pressed_keys: Vec<String> = key_events
		.clone()
		.filter(|(key_state, _)| *key_state == KeyState::Pressed)
		.map(|(_, key)| key)
		.collect();

	if !pressed_keys.is_empty() {
		println!("pressed");
		keys.extend(pressed_keys);
	}

	let released_keys: Vec<String> = key_events
		.filter(|(key_state, _)| *key_state == KeyState::Released)
		.map(|(_, key)| key)
		.collect();

	if !released_keys.is_empty() {
		println!("released");
		return Some(keys);
	}

	if !keys.is_empty() {
		Some(keys) // Return the list of all pressed keys
	} else {
		None
	}
}
